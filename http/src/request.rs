use crate::{Error, Result};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, patch};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

pub fn router() -> Router {
    Router::new()
        .route("/v1/requests", get(list_requests).post(create_request))
        .route("/v1/requests/:requestId/binding", patch(binding_request))
        .route("/v1/requests/:requestId/accept", patch(accept_helper))
        .route("/v1/requests/:requestId/done", patch(mark_done))
}

#[derive(Deserialize, Validate)]
struct CreateRequestRequest {
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 10))]
    description: String,
    #[validate(range(min = 1))]
    price: i64,
    #[validate(range(min = 1))]
    requester_id: i32,
    helper_id: i32,
}

#[serde_with::serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    id: i32,
    title: std::option::Option<std::string::String>,
    description: std::option::Option<std::string::String>,
    price: std::option::Option<i64>,
    requester_id: std::option::Option<i32>,
    helper_id: std::option::Option<i32>,
    status: String,
}

async fn create_request(
    db: Extension<PgPool>,
    Json(req): Json<CreateRequestRequest>,
) -> Result<Json<Request>> {
    req.validate()?;
    let status = if req.helper_id > 0 { "binding" } else { "new" };
    let request = sqlx::query_as!(
        Request,
        r#"
            with inserted_request as (
                insert into requests(
                    title,
                    description,
                    price,
                    requester_id,
                    helper_id,
                    status)
                values($1, $2, $3, $4, $5, $6)
                returning id,
                title,
                description,
                price,
                requester_id,
                helper_id,
                status
            )
            select id,
            title,
            description,
            price,
            requester_id,
            helper_id,
            status
            from inserted_request
        "#,
        req.title,
        req.description,
        req.price,
        req.requester_id,
        req.helper_id,
        status,
    )
    .fetch_one(&*db)
    .await?;

    Ok(Json(request))
}

async fn list_requests(db: Extension<PgPool>) -> Result<Json<Vec<Request>>> {
    let requests = sqlx::query_as!(
        Request,
        r#"
            select id,
            title,
            description,
            status,
            price,
            requester_id,
            helper_id
            from requests
            where deleted_at is null
            "#,
    )
    .fetch_all(&*db)
    .await?;

    Ok(Json(requests))
}

#[serde_with::serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HelperRequest {
    id: i32,
    request_id: std::option::Option<i32>,
    helper_id: std::option::Option<i32>,
}

#[derive(Deserialize, Validate)]
struct BindingRequestRequest {
    #[validate(range(min = 1))]
    helper_id: i32,
}

async fn binding_request(
    db: Extension<PgPool>,
    Path(request_id): Path<i32>,
    Json(req): Json<BindingRequestRequest>,
) -> Result<StatusCode> {
    req.validate()?;
    sqlx::query!(
        r#"
            insert into helper_requests(
                request_id,
                helper_id)
            values($1, $2)
        "#,
        request_id,
        req.helper_id,
    )
    .execute(&*db)
    .await
    .map_err(|e| match e {
        _ => Error::Sqlx(e.into()),
    })?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, Validate)]
struct AcceptHelperRequest {
    #[validate(range(min = 1))]
    helper_id: i32,
}

async fn accept_helper(
    db: Extension<PgPool>,
    Path(request_id): Path<i32>,
    Json(req): Json<AcceptHelperRequest>,
) -> Result<StatusCode> {
    req.validate()?;

    sqlx::query!(
        r#"
            update requests
            set helper_id = $1, status = $2
            where id = $3
        "#,
        req.helper_id,
        "in_progres",
        request_id,
    )
    .execute(&*db)
    .await
    .map_err(|e| match e {
        _ => Error::Sqlx(e.into()),
    })?;

    Ok(StatusCode::NO_CONTENT)
}

async fn mark_done(db: Extension<PgPool>, Path(request_id): Path<i32>) -> Result<StatusCode> {
    sqlx::query!(
        r#"
            update requests
            set status = $1
            where id = $2
        "#,
        "done",
        request_id,
    )
    .execute(&*db)
    .await
    .map_err(|e| match e {
        _ => Error::Sqlx(e.into()),
    })?;

    Ok(StatusCode::NO_CONTENT)
}
