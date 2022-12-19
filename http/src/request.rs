use crate::Result;
use axum::routing::get;
use axum::routing::post;
use axum::Extension;
use axum::Json;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use validator::Validate;

pub fn router() -> Router {
    Router::new().route("/v1/requests", get(list_requests).post(create_request))
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
            select id, title, description, status, price, requester_id, helper_id
            from requests
            where deleted_at is null
            "#,
    )
    .fetch_all(&*db)
    .await?;

    Ok(Json(requests))
}
