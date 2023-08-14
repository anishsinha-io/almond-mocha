use sqlx::{types::Uuid, Acquire, Executor, Postgres, Transaction};
use std::error::Error;

use crate::app::dto::{
    CreateUser, DeleteUser, EditUser, GetUserByEmail, GetUserById, HashAlgorithm,
};
use crate::app::storage::entities::{User, UserWithCredentials};

pub async fn create_user<'a>(
    executor: impl Executor<'a, Database = Postgres> + Acquire<'a, Database = Postgres>,
    data: CreateUser,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut txn: Transaction<'_, Postgres> = executor.begin().await?;

    let user = sqlx::query!(
        r#"insert into jen.users (first_name, last_name, email, username, image_uri) values ($1, $2, $3, $4, $5) on conflict(email) do nothing returning id"#,
        data.first_name, data.last_name, data.email, data.username,data.image_uri
    )
    .fetch_one(&mut *txn)
    .await?;

    if let (Some(hash), Some(alg)) = (data.hashed_password, data.algorithm) {
        sqlx::query(r#"insert into jen.user_credentials (user_id, credential_hash, alg) values ($1, $2, $3)"#)
            .bind(user.id)
            .bind(hash)
            .bind(alg)
            .execute(&mut *txn)
            .await?;
    }

    txn.commit().await?;

    Ok(user.id.to_string())
}

pub async fn get_user_by_id<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetUserById,
) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let user = sqlx::query_as!(User, r#"select id, first_name, last_name, email, username, image_uri, created_at, updated_at from jen.users where id=$1"#, id).fetch_optional(executor).await?;
    Ok(user)
}

#[allow(unused)]
pub async fn get_user_by_email<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetUserByEmail,
) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
    let user = sqlx::query_as!(User, r#"select id, first_name, last_name, email, username, image_uri, created_at, updated_at from jen.users where email=$1"#, data.email).fetch_optional(executor).await?;
    Ok(user)
}

pub async fn get_user_with_credentials_by_email<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetUserByEmail,
) -> Result<Option<UserWithCredentials>, Box<dyn Error + Send + Sync>> {
    let user = sqlx::query_as!(
        UserWithCredentials,
        r#"select users.id, first_name, last_name, email, username, image_uri, 
           jen.user_credentials.credential_hash, jen.user_credentials.alg as "alg!: HashAlgorithm", 
           users.created_at, users.updated_at from jen.users join jen.user_credentials 
           on users.id=user_credentials.user_id and email=$1"#,
        data.email
    )
    .fetch_optional(executor)
    .await?;
    Ok(user)
}

pub async fn edit_user<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: EditUser,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    sqlx::query!(
        "update jen.users set first_name=$2, last_name=$3, username=$4, image_uri=$5 where id=$1",
        id,
        data.first_name,
        data.last_name,
        data.username,
        data.image_uri
    )
    .execute(executor)
    .await?;
    Ok(())
}

#[allow(unused)]
pub async fn delete_user<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: DeleteUser,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    sqlx::query(r#"delete from jen.users where id=$1"#)
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::app::{
        auth::CredentialManager, dto::HashAlgorithm, storage::postgres::create_pool, util,
    };

    use super::*;

    #[tokio::test]
    pub async fn test_users() {
        util::test_util::init();
        let pool = create_pool(5).await.unwrap();

        let mut txn = pool.begin().await.expect("error starting transaction");

        let random_suffix = util::rng::random_string(4);

        let email = format!("jennycho35-{random_suffix}@gmail.com");
        let mut nonexistent = get_user_by_email(
            &mut *txn,
            GetUserByEmail {
                email: email.clone(),
            },
        )
        .await
        .expect("error getting user by email");

        assert!(nonexistent.is_none());

        let manager = CredentialManager::new(HashAlgorithm::Argon2);
        let hash = manager.create_hash(b"jennysinha").unwrap();

        let new_user = CreateUser {
            first_name: "Jenny".to_owned(),
            last_name: "Sinha".to_owned(),
            email: email.clone(),
            username: format!("jennysinha-{random_suffix}"),
            image_uri: "https://assets.anishsinha.com/jenny".to_owned(),
            hashed_password: Some(hash.to_owned()),
            algorithm: Some(HashAlgorithm::Argon2),
        };

        let new_user = create_user(&mut *txn, new_user)
            .await
            .expect("error creating new user");

        let existing_user = get_user_by_email(
            &mut *txn,
            GetUserByEmail {
                email: email.clone(),
            },
        )
        .await
        .unwrap();

        assert!(existing_user.is_some());
        assert_eq!(existing_user.unwrap().id.to_string(), new_user);

        let by_id = get_user_by_id(
            &mut *txn,
            GetUserById {
                id: new_user.clone(),
            },
        )
        .await
        .expect("error getting user by id");

        assert!(by_id.is_some());
        assert_eq!(by_id.unwrap().id.to_string(), new_user.clone());

        edit_user(
            &mut *txn,
            EditUser {
                id: new_user.clone(),
                first_name: "Jenny".to_owned(),
                last_name: "Sinha".to_owned(),
                username: "jen_sinha".to_owned(),
                image_uri: "https://assets.anishsinha.io/jen".to_owned(),
            },
        )
        .await
        .expect("error editing user");

        let by_email = get_user_by_email(
            &mut *txn,
            GetUserByEmail {
                email: email.clone(),
            },
        )
        .await
        .expect("error getting user by email");

        assert!(by_email.is_some());
        assert_eq!(by_email.unwrap().username, "jen_sinha");

        delete_user(
            &mut *txn,
            DeleteUser {
                id: new_user.clone(),
            },
        )
        .await
        .expect("error deleting user");

        nonexistent = get_user_by_id(
            &mut *txn,
            GetUserById {
                id: new_user.clone(),
            },
        )
        .await
        .expect("error fetching user by id");

        assert!(nonexistent.is_none());
        txn.rollback()
            .await
            .expect("error rolling back transaction");
    }
}
