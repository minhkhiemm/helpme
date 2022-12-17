use anyhow::Context;
use axum::{Extension, Router};
use sqlx::PgPool;

mod error;
mod request;

pub use error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

pub fn app(db: PgPool) -> Router {
    Router::new().merge(request::router()).layer(Extension(db))
}

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app(db).into_make_service())
        .await
        .context("failed to serve API")
}
