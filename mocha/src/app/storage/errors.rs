use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum StorageError {
    #[display(fmt = "error starting redis session")]
    RedisStartSession,
    #[display(fmt = "error ending redis session")]
    RedisEndSession,
    PgStartSession,
    PgEndSession,
    RedisGetSession,
    PgGetSession,
    NotFound,
}
