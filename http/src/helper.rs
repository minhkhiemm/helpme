use crate::Result;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde::Serialize;
use sqlx::PgPool;

pub fn router() -> Router {
    Router::new().route("/v1/helpers", get(list_helpers))
}

#[serde_with::serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Helper {
    id: i32,
    name: std::option::Option<std::string::String>,
    phone_number: std::option::Option<std::string::String>,
    email: std::option::Option<std::string::String>,
}

async fn list_helpers(db: Extension<PgPool>) -> Result<Json<Vec<Helper>>> {
    let helpers = sqlx::query_as!(
        Helper,
        r#"
            select id,
            name,
            phone_number,
            email
            from helpers
            where deleted_at is null
        "#
    )
    .fetch_all(&*db)
    .await?;

    Ok(Json(helpers))
}
