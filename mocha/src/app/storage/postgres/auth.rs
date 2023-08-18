use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{Acquire, Executor, Postgres, QueryBuilder};
use std::error::Error;
use uuid::Uuid;

use crate::app::{
    dto::auth::{
        CreatePermission, CreateRole, CreateSession, DeletePermission, DeleteSession,
        EditPermission, GetPermissionById, GetRoleById, GetSessionById,
    },
    entities::auth::{Permission, Role, Session},
    storage::errors::StorageError,
};

pub async fn start_session<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: CreateSession,
) -> Result<String, StorageError> {
    let user_id = Uuid::parse_str(&data.user_id).map_err(|_| {
        log::error!("error converting string to uuid");
        StorageError::PgStartSession
    })?;
    let session = sqlx::query!(
        r#"insert into jen.sessions (user_id, data) values ($1, $2) returning id"#,
        user_id,
        data.data
    )
    .fetch_one(executor)
    .await
    .map_err(|_| StorageError::PgStartSession)?;

    Ok(session.id.to_string())
}

pub async fn end_session<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: DeleteSession,
) -> Result<(), StorageError> {
    let session_id = Uuid::parse_str(&data.id).map_err(|_| {
        log::error!("error converting string (session id) to uuid");
        StorageError::PgEndSession
    })?;
    sqlx::query("delete from jen.sessions where id=$1")
        .bind(session_id)
        .execute(executor)
        .await
        .map_err(|_| StorageError::PgEndSession)?;
    Ok(())
}

pub async fn get_session<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetSessionById,
) -> Result<Session, StorageError> {
    let session_id = Uuid::parse_str(&data.id).map_err(|_| {
        log::error!("error converting string (session id) to uuid");
        StorageError::PgEndSession
    })?;
    let session = sqlx::query_as!(
        Session,
        "select id, user_id, data, created_at, updated_at from jen.sessions where id=$1",
        session_id
    )
    .fetch_one(executor)
    .await
    .map_err(|_| StorageError::PgGetSession)?;
    Ok(session)
}

pub async fn create_permission<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: CreatePermission,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let sql = "insert into jen.permissions (permission_name, permission_description) values ($1, $2) returning id";
    let (permission_id,): (Uuid,) = sqlx::query_as(sql)
        .bind(data.name)
        .bind(data.description)
        .fetch_one(executor)
        .await?;
    Ok(permission_id.to_string())
}

pub async fn get_permission_by_id<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetPermissionById,
) -> Result<Option<Permission>, Box<dyn Error + Send + Sync>> {
    let permission_id = Uuid::parse_str(&data.id)?;
    let maybe_permission = sqlx::query_as!(
        Permission,
        "select id, permission_name, permission_description, created_at, updated_at from 
               jen.permissions where id=$1",
        permission_id,
    )
    .fetch_optional(executor)
    .await?;
    Ok(maybe_permission)
}

pub async fn edit_permission<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: EditPermission,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let permission_id = Uuid::parse_str(&data.id)?;
    let sql = "update jen.permissions set permission_description=$1 where id=$2";
    let res = sqlx::query(sql)
        .bind(data.description)
        .bind(permission_id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete_permission<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: DeletePermission,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let permission_id = Uuid::parse_str(&data.id)?;
    let sql = "delete from jen.permissions where id=$1";
    let res = sqlx::query(sql)
        .bind(permission_id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

pub async fn create_role<'a>(
    executor: impl Executor<'a, Database = Postgres> + Acquire<'a, Database = Postgres>,
    data: CreateRole,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut txn = executor.begin().await?;
    let sql = "insert into jen.roles (role_name, role_description) values ($1, $2) returning id";
    let (role_id,): (Uuid,) = sqlx::query_as(sql)
        .bind(data.name)
        .bind(data.description)
        .fetch_one(&mut *txn)
        .await?;

    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("insert into jen.permissions (permission_name, permission_description) ");
    builder.push_values(data.permissions.into_iter(), |mut b, p| {
        b.push_bind(p.name).push_bind(p.description);
    });

    builder.push("returning id");
    let query = builder.build_query_as();

    let new_permission_ids: Vec<(Uuid,)> = query.fetch_all(&mut *txn).await?;

    let mut mapping_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("insert into jen.role_permission_mappings (role_id, permission_id) ");

    mapping_builder.push_values(new_permission_ids.into_iter(), |mut b, (id,)| {
        b.push_bind(role_id).push_bind(id);
    });

    let mapping_query = mapping_builder.build();
    mapping_query.execute(&mut *txn).await?;

    txn.commit().await?;
    Ok(role_id.to_string())
}

pub async fn get_role<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetRoleById,
) -> Result<Option<Role>, Box<dyn Error + Send + Sync>> {
    let sql = "select id, role_name, role_description, created_at, updated_at,
               (select coalesce((select json_agg(role_permissions) from jen.permissions 
               role_permissions where (exists (select 1 from jen.role_permission_mappings 
               where (jen.role_permission_mappings.role_id=$1) and 
               (role_permissions.id = jen.role_permission_mappings.permission_id)))), '[]'::json) 
			   as role_permissions) as permissions from jen.roles;";

    type RoleTuple = (Uuid, String, String, DateTime<Utc>, DateTime<Utc>, Value);

    let role_id = Uuid::parse_str(&data.id)?;
    let maybe_role: Option<RoleTuple> = sqlx::query_as(sql)
        .bind(role_id)
        .fetch_optional(executor)
        .await?;

    match maybe_role {
        Some((id, role_name, role_description, created_at, updated_at, permissions)) => {
            Ok(Some(Role {
                id,
                role_name,
                role_description,
                created_at,
                updated_at,
                permissions: serde_json::from_value::<Vec<Permission>>(permissions)?,
            }))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use crate::app::{storage::postgres, util};

    use super::*;

    #[tokio::test]
    pub async fn test_create_permission() {
        util::test_util::init();
        let pool = postgres::create_pool(5).await.unwrap();
        let mut txn = pool.begin().await.unwrap();

        let new_permission_id = create_permission(
            &mut *txn,
            CreatePermission {
                name: "posts:create".to_owned(),
                description: "allow a user to create a post".to_owned(),
            },
        )
        .await
        .unwrap();

        let permission = get_permission_by_id(
            &mut *txn,
            GetPermissionById {
                id: new_permission_id.clone(),
            },
        )
        .await
        .unwrap();

        println!("{:#?}", permission);

        let x = edit_permission(
            &mut *txn,
            EditPermission {
                id: new_permission_id.clone(),
                description: "permission to create a post".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(x, 1);

        let y = delete_permission(
            &mut *txn,
            DeletePermission {
                id: new_permission_id.clone(),
            },
        )
        .await
        .unwrap();

        assert_eq!(y, 1);

        txn.rollback().await.unwrap();
    }

    #[tokio::test]
    pub async fn test_roles() {
        util::test_util::init();
        let pool = postgres::create_pool(5).await.unwrap();
        let mut txn = pool.begin().await.unwrap();

        let new_role_id = create_role(
            &mut *txn,
            CreateRole {
                name: "admin".to_owned(),
                description: "default admin roles".to_owned(),
                permissions: vec![
                    CreatePermission {
                        name: "posts:create".to_owned(),
                        description: "permission required to create a post".to_owned(),
                    },
                    CreatePermission {
                        name: "posts:delete".to_owned(),
                        description: "permission required to delete a post".to_owned(),
                    },
                ],
            },
        )
        .await
        .unwrap();
        println!("{new_role_id}");

        let role = get_role(
            &mut *txn,
            GetRoleById {
                id: new_role_id.clone(),
            },
        )
        .await
        .unwrap();
        println!("{:#?}", role);

        txn.commit().await.unwrap();
    }
}
