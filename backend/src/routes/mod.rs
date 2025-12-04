use actix_web::web;
use diesel::SqliteConnection;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, sync_connection_wrapper::SyncConnectionWrapper};
use utoipa::OpenApi;

use crate::{DbPool, models::DbConnection};

pub mod posts;
pub mod api_models;
pub mod users;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    posts::init(cfg);
    users::init(cfg);
}

pub fn docs() -> utoipa::openapi::OpenApi {
    posts::ApiDoc::openapi()
        .merge_from(users::ApiDoc::openapi())
}


async fn get_conn(
    pool: web::Data<DbPool>
) -> DbConnection {
    pool.get()
        .await
        .expect("Pool")
}