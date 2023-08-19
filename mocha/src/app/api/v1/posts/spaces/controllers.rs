use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use actix_web_grants::proc_macro::has_permissions;

use crate::app::{
    dto::{
        pagination::{PaginationLimits, SpacePaginationOptions, TagPaginationOptions},
        spaces::{CreateSpace, DeleteSpace, EditSpace, EditSpaceInfo, GetSpaceById},
        tags::{
            CreateTag, CreateTagInfo, DeleteTag, EditTag, EditTagInfo, GetTagById, GetTagsBySpace,
        },
    },
    errors::AppError,
    state::AppState,
    storage::postgres,
};

#[has_permissions("spaces:create")]
pub async fn create_space(
    state: Data<AppState>,
    data: Json<CreateSpace>,
) -> actix_web::Result<HttpResponse, AppError> {
    let dto = data.into_inner();
    let _ = postgres::spaces::create_space(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"msg": "successfully created new space"})))
}

pub async fn get_space(
    state: Data<AppState>,
    space: Path<String>,
) -> actix_web::Result<HttpResponse, AppError> {
    let dto = GetSpaceById {
        id: space.into_inner(),
    };

    let space = postgres::spaces::get_space_by_id(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    match space {
        Some(_) => Ok(HttpResponse::Ok().json(serde_json::json!({ "space": space }))),
        None => Err(AppError::NotFound),
    }
}

pub async fn get_spaces(
    state: Data<AppState>,
    pagination: Json<PaginationLimits<SpacePaginationOptions>>,
) -> actix_web::Result<HttpResponse, AppError> {
    let spaces = postgres::spaces::get_spaces(&state.storage_layer.pg, pagination.into_inner())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "spaces": spaces })))
}

#[has_permissions("spaces:edit")]
pub async fn edit_space(
    state: Data<AppState>,
    space: Path<String>,
    info: Json<EditSpaceInfo>,
) -> actix_web::Result<HttpResponse, AppError> {
    let data = info.into_inner();

    let dto = EditSpace {
        id: space.into_inner(),
        space_name: data.space_name,
        bio: data.bio,
    };
    postgres::spaces::edit_space(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"msg": "successfully edited space"})))
}

#[has_permissions("spaces:delete")]
pub async fn delete_space(
    state: Data<AppState>,
    space: Path<String>,
) -> actix_web::Result<HttpResponse, AppError> {
    let dto = DeleteSpace {
        id: space.into_inner(),
    };
    postgres::spaces::delete_space(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok(HttpResponse::NoContent().finish())
}

#[has_permissions("tags:create")]
pub async fn create_tag(
    state: Data<AppState>,
    space: Path<String>,
    data: Json<CreateTagInfo>,
) -> actix_web::Result<HttpResponse, AppError> {
    let info = data.into_inner();
    let dto = CreateTag {
        space_id: space.into_inner(),
        name: info.name,
        description: info.description,
    };
    let _ = postgres::spaces::create_tag(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"msg": "succesfully created new tag"})))
}

pub async fn get_tag(
    state: Data<AppState>,
    tag: Path<String>,
) -> actix_web::Result<HttpResponse, AppError> {
    let dto = GetTagById {
        id: tag.into_inner(),
    };

    let maybe_tag = postgres::spaces::get_tag_by_id(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    match maybe_tag {
        Some(tag) => Ok(HttpResponse::Ok().json(serde_json::json!({ "tag": tag }))),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "tag not found"}))),
    }
}

pub async fn get_tags(
    state: Data<AppState>,
    space: Path<String>,
    pagination: Json<PaginationLimits<TagPaginationOptions>>,
) -> actix_web::Result<HttpResponse, AppError> {
    let tags = postgres::spaces::get_tags(
        &state.storage_layer.pg,
        GetTagsBySpace {
            space_id: space.into_inner(),
        },
        pagination.into_inner(),
    )
    .await
    .map_err(|_| AppError::InternalServerError)?;

    match tags.items.is_empty() {
        true => Ok(HttpResponse::NotFound()
            .json(serde_json::json!({"msg": "no tags found", "tags": tags}))),
        false => Ok(HttpResponse::Ok().json(serde_json::json!({ "tags": tags }))),
    }
}

#[has_permissions("tags:edit")]
pub async fn edit_tag(
    state: Data<AppState>,
    tag: Path<String>,
    data: Json<EditTagInfo>,
) -> actix_web::Result<HttpResponse, AppError> {
    let info = data.into_inner();
    postgres::spaces::edit_tag(
        &state.storage_layer.pg,
        EditTag {
            id: tag.into_inner(),
            name: info.name,
            description: info.description,
        },
    )
    .await
    .map_err(|_| AppError::InternalServerError)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"msg": "tag successfully edited"})))
}

#[has_permissions("tags:delete")]
pub async fn delete_tag(
    state: Data<AppState>,
    tag: Path<String>,
) -> actix_web::Result<HttpResponse, AppError> {
    let dto = DeleteTag {
        id: tag.into_inner(),
    };
    postgres::spaces::delete_tag(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok(HttpResponse::NoContent().finish())
}
