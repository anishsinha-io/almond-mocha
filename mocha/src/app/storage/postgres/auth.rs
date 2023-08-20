use serde_json::Value;
use sqlx::{Acquire, Executor, Postgres, QueryBuilder};
use std::{collections::HashSet, error::Error};
use uuid::Uuid;

use crate::app::{
    dto::auth::{
        AddRoleToUser, AttachInlinePermission, CreatePermission, CreateRole, CreateSession,
        DeletePermission, DeleteRole, DeleteSession, EditPermission, EditRole, GetPermissionById,
        GetRoleById, GetSessionById, GetUserRbac,
    },
    entities::auth::{Permission, Role, RoleWithPermissions, Session, UserAccess, UserRbac},
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

pub async fn create_permissions<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: Vec<CreatePermission>,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("insert into jen.permissions (permission_name, permission_description)");
    builder.push_values(data.into_iter(), |mut b, p| {
        b.push_bind(p.name).push_bind(p.description);
    });

    builder.push("returning id");
    let query = builder.build_query_as();
    let rows: Vec<(Uuid,)> = query.fetch_all(executor).await?;

    let new_permission_ids: Vec<String> = rows.into_iter().map(|r| r.0.to_string()).collect();

    Ok(new_permission_ids)
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
) -> Result<Option<RoleWithPermissions>, Box<dyn Error + Send + Sync>> {
    let sql = "select id, role_name, role_description, created_at, updated_at,
               (select coalesce((select json_agg(role_permissions) from jen.permissions 
               role_permissions where (exists (select 1 from jen.role_permission_mappings 
               where (jen.role_permission_mappings.role_id=$1) and 
               (role_permissions.id = jen.role_permission_mappings.permission_id)))), '[]'::json) 
			   as role_permissions) as permissions from jen.roles where id=$1;";

    let role_id = Uuid::parse_str(&data.id)?;
    match sqlx::query_as(sql)
        .bind(role_id)
        .fetch_optional(executor)
        .await?
    {
        Some((id, role_name, role_description, created_at, updated_at, permissions)) => {
            Ok(Some(RoleWithPermissions {
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

pub async fn edit_role<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: EditRole,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let sql = "update jen.roles set role_name=$1, role_description=$2 where id=$3";

    let role_id = Uuid::parse_str(&data.id)?;
    let res = sqlx::query(sql)
        .bind(data.name)
        .bind(data.description)
        .bind(role_id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete_role<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: DeleteRole,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let sql = "delete from jen.roles where id=$1";

    let role_id = Uuid::parse_str(&data.id)?;
    let res = sqlx::query(sql).bind(role_id).execute(executor).await?;
    Ok(res.rows_affected())
}

pub async fn attach_inline_permissions<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: Vec<AttachInlinePermission>,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("insert into jen.user_permission_mappings (user_id, permission_id)");
    builder.push_values(data.into_iter(), |mut b, mapping| {
        if let (Ok(id), Ok(user_id)) = (
            Uuid::parse_str(&mapping.id),
            Uuid::parse_str(&mapping.user_id),
        ) {
            b.push_bind(user_id).push_bind(id);
        }
    });

    let query = builder.build();
    let res = query.execute(executor).await?;
    Ok(res.rows_affected())
}

// TODO: Reevaluate this function and the one above (attach_inline_permissions). Currently the
// behavior is to insert all valid mappings. I'm not sure how I feel about this (maybe any input
// which contains any invalid mapping should fail?). Currently the function just ignores invalid
// mappings.
pub async fn add_roles_to_user<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: Vec<AddRoleToUser>,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("insert into jen.user_role_mappings (user_id, role_id)");
    builder.push_values(data.into_iter(), |mut b, mapping| {
        if let (Ok(id), Ok(user_id)) = (
            Uuid::parse_str(&mapping.id),
            Uuid::parse_str(&mapping.user_id),
        ) {
            b.push_bind(user_id).push_bind(id);
        }
    });
    let query = builder.build();
    let res = query.execute(executor).await?;
    Ok(res.rows_affected())
}

pub async fn get_user_access<'a>(
    executor: impl Executor<'a, Database = Postgres> + Acquire<'a, Database = Postgres>,
    data: GetUserRbac,
) -> Result<UserAccess, Box<dyn Error + Send + Sync>> {
    let sql = "select array_to_json(
	            (select array_agg(user_roles) from (
		            select role_id as id, roles.role_name, roles.role_description, roles.created_at, roles.updated_at from 
		            jen.user_role_mappings join 
		            jen.roles on roles.id=role_id and 
		            user_id=$1
	            ) user_roles)
               )::jsonb as roles, array_to_json(jen.get_user_permissions($1))::jsonb as permissions;";

    let user_id = Uuid::parse_str(&data.user_id)?;
    let (json_roles, json_permissions): (Value, Value) = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_one(executor)
        .await?;

    let roles = serde_json::from_value::<Vec<Role>>(json_roles)?;
    let permissions = serde_json::from_value::<Vec<Permission>>(json_permissions)?;

    Ok(UserAccess { roles, permissions })
}

pub async fn get_user_rbac<'a>(
    executor: impl Executor<'a, Database = Postgres> + Acquire<'a, Database = Postgres>,
    data: GetUserRbac,
) -> Result<UserRbac, Box<dyn Error + Send + Sync>> {
    let mut txn = executor.begin().await?;

    // NOTE: jen.get_role_name is a plpgsql function that could be replaced by a subquery if
    // database systems had to be migrated
    let roles_query =
        "select role_id, jen.get_role_name(role_id) as role_name from jen.user_role_mappings where user_id=$1";
    let user_id = Uuid::parse_str(&data.user_id)?;
    let roles: Vec<(Uuid, String)> = sqlx::query_as(roles_query)
        .bind(user_id)
        .fetch_all(&mut *txn)
        .await?;

    let role_membership = roles.iter().map(|(_, name)| name.clone()).collect();

    // let mut permissions: Vec<(String, String)> = Vec::new();
    let mut permissions_set: HashSet<(Uuid, String)> = HashSet::new();

    for (role_id, _) in roles {
        if let Some(role) = get_role(
            &mut *txn,
            GetRoleById {
                id: role_id.to_string(),
            },
        )
        .await?
        {
            role.permissions.iter().for_each(|permission| {
                permissions_set.insert((permission.id, permission.permission_name.to_owned()));
            });
        };
    }

    let inline_permissions_query = "select permission_id, permission_name from 
                                    jen.user_permission_mappings join jen.permissions on 
                                    permission_id=permissions.id and 
                                    user_permission_mappings.user_id=$1";

    let inline_permissions: Vec<(Uuid, String)> = sqlx::query_as(inline_permissions_query)
        .bind(user_id)
        .fetch_all(&mut *txn)
        .await?;

    inline_permissions.into_iter().for_each(|(id, name)| {
        permissions_set.insert((id, name));
    });

    let permissions: Vec<String> = permissions_set.into_iter().map(|(_, name)| name).collect();

    txn.commit().await?;

    Ok(UserRbac {
        role_membership,
        permissions,
    })
}

#[cfg(test)]
mod tests {
    use crate::app::{
        auth::CredentialManager, dto::users::CreateUser, storage::postgres, types::HashAlgorithm,
        util,
    };

    use super::*;

    #[tokio::test]
    pub async fn test_create_permission() {
        util::test_util::init();
        let pool = postgres::create_pool(5).await.unwrap();
        let mut txn = pool.begin().await.unwrap();

        let new_permission_id = create_permission(
            &mut *txn,
            CreatePermission {
                name: "test1".to_owned(),
                description: "test permission".to_owned(),
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
                        name: "test1".to_owned(),
                        description: "permission required to create a post".to_owned(),
                    },
                    CreatePermission {
                        name: "test2".to_owned(),
                        description: "permission required to delete a post".to_owned(),
                    },
                ],
            },
        )
        .await
        .unwrap();

        let role = get_role(
            &mut *txn,
            GetRoleById {
                id: new_role_id.clone(),
            },
        )
        .await
        .unwrap();
        println!("{:#?}", role);

        edit_role(
            &mut *txn,
            EditRole {
                id: new_role_id.clone(),
                name: "Admin".to_owned(),
                description: "Default admin roles".to_owned(),
            },
        )
        .await
        .unwrap();

        delete_role(
            &mut *txn,
            DeleteRole {
                id: new_role_id.clone(),
            },
        )
        .await
        .unwrap();

        let nonexistent = get_role(
            &mut *txn,
            GetRoleById {
                id: new_role_id.clone(),
            },
        )
        .await
        .unwrap();
        assert!(nonexistent.is_none());

        txn.rollback().await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rbac() {
        util::test_util::init();
        let pool = postgres::create_pool(5).await.unwrap();
        let mut txn = pool.begin().await.unwrap();

        let random_suffix = util::rng::random_string(4);

        let email = format!("jennycho35-{random_suffix}@gmail.com");

        let manager = CredentialManager::new(HashAlgorithm::Argon2);
        let hash = manager.create_hash(b"jennysinha").unwrap();

        let new_user = CreateUser {
            first_name: "Jenny".to_owned(),
            last_name: "Sinha".to_owned(),
            email: email.clone(),
            username: format!("jennysinha-{random_suffix}"),
            bio: "amazing programmer and musician".to_owned(),
            image_uri: "https://assets.anishsinha.com/jenny".to_owned(),
            hashed_password: Some(hash.to_owned()),
            algorithm: Some(HashAlgorithm::Argon2),
        };

        let new_user = postgres::users::create_user(&mut *txn, new_user)
            .await
            .expect("error creating new user");

        let new_permissions = create_permissions(
            &mut *txn,
            vec![
                CreatePermission {
                    name: "p1-name".to_owned(),
                    description: "p1-bio".to_owned(),
                },
                CreatePermission {
                    name: "p2-name".to_owned(),
                    description: "p2-bio".to_owned(),
                },
                CreatePermission {
                    name: "p3-name".to_owned(),
                    description: "p3-bio".to_owned(),
                },
            ],
        )
        .await
        .unwrap();

        attach_inline_permissions(
            &mut *txn,
            new_permissions
                .into_iter()
                .map(|p| AttachInlinePermission {
                    id: p,
                    user_id: new_user.clone(),
                })
                .collect(),
        )
        .await
        .unwrap();

        let role = create_role(
            &mut *txn,
            CreateRole {
                name: "r1".to_owned(),
                description: "r1-bio".to_owned(),
                permissions: vec![CreatePermission {
                    name: "rp1".to_string(),
                    description: "rp1-bio".to_string(),
                }],
            },
        )
        .await
        .unwrap();

        add_roles_to_user(
            &mut *txn,
            vec![AddRoleToUser {
                id: role,
                user_id: new_user.clone(),
            }],
        )
        .await
        .unwrap();

        let rbac = get_user_rbac(
            &mut *txn,
            GetUserRbac {
                user_id: new_user.clone(),
            },
        )
        .await
        .unwrap();

        let access = get_user_access(
            &mut *txn,
            GetUserRbac {
                user_id: new_user.clone(),
            },
        )
        .await
        .unwrap();
        println!("{:#?}", access);

        txn.rollback().await.unwrap();
    }
}
