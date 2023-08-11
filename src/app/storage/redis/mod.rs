use derive_more::{Display, Error};
use mobc::{Connection, Pool};
use mobc_redis::redis::AsyncCommands;
use mobc_redis::{redis, RedisConnectionManager};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

pub mod auth;

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConn = Connection<RedisConnectionManager>;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub async fn create_pool() -> Result<RedisPool, Box<dyn Error + Send + Sync>> {
    let conn_string = env!("REDIS_URL");
    let client = redis::Client::open(conn_string)?;
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager);
    Ok(pool)
}

#[derive(Debug, Display, Error)]
pub enum CacheError {
    #[display(fmt = "invalid data")]
    InvalidData,
    #[display(fmt = "cache miss")]
    Miss,
    #[display(fmt = "server error")]
    ServerError,
}

pub async fn set_json<'a, T>(conn: &mut RedisConn, key: &str, value: T) -> Result<(), CacheError>
where
    T: Serialize + Deserialize<'a>,
{
    let bytes = serde_json::to_string(&value)
        .map(|s| s.as_bytes().to_vec())
        .map_err(|_| CacheError::InvalidData)?;

    conn.set(key.to_owned(), bytes)
        .await
        .map_err(|_| CacheError::ServerError)?;

    Ok(())
}

pub async fn get_json<'a, T>(conn: &mut RedisConn, key: &str) -> Result<Option<T>, CacheError>
where
    T: Serialize + DeserializeOwned,
{
    let json: Option<String> = conn.get(key).await.map_err(|_| CacheError::ServerError)?;
    match json {
        Some(value) => {
            let s: T = serde_json::from_str(&value).map_err(|_| CacheError::Miss)?;
            Ok(Some(s))
        }
        None => Ok(None),
    }
}

#[inline]
pub async fn delete(
    conn: &mut Connection<RedisConnectionManager>,
    key: &str,
) -> Result<(), CacheError> {
    conn.del(key).await.map_err(|_| CacheError::ServerError)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestStruct {
        pub name: String,
        pub age: u32,
    }

    #[tokio::test]
    pub async fn test_conn() {
        let pool = create_pool().await.expect("error creating redis pool");
        let mut conn = pool.get().await.expect("error taking connection from pool");

        let t = TestStruct {
            name: "Jenny Cho".to_owned(),
            age: 21,
        };

        set_json(&mut conn, "test", t)
            .await
            .expect("error inserting value");

        let v1: TestStruct = get_json(&mut conn, "test")
            .await
            .expect("error retrieving value")
            .unwrap();

        println!("{:#?}", v1);

        set_json(
            &mut conn,
            "key1",
            HashMap::<String, String>::from([("k1".to_owned(), "v1".to_owned())]),
        )
        .await
        .expect("error inserting hashmap");

        let v2: HashMap<String, String> = get_json(&mut conn, "key1")
            .await
            .expect("error retrieving value")
            .unwrap();

        println!("{:#?}", v2);

        set_json(&mut conn, "key2", vec![4f64, 5f64, 6f64])
            .await
            .expect("error inserting float vector");

        let v3: Vec<f64> = get_json(&mut conn, "key2")
            .await
            .expect("error retrieving value")
            .unwrap();

        println!("{:#?}", v3);

        delete(&mut conn, "key1").await.expect("error deleting key");
    }
}
