use actix_web::{HttpResponse, Responder, web};
use serde::Serialize;
use utoipa::OpenApi;

use crate::{DbPool, models::{DbConnection, DbError}, routes::api_models::common::ApiResponse};

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

fn handle_db_result<T: Serialize>(result: Result<T, DbError>) -> impl Responder {
    match result {
        Ok(body) => HttpResponse::Ok().json(ApiResponse::success(body)),
        Err(e) => {
            let (msg, code) = e.handle();
            e.to_http_response()
                .json(ApiResponse::<()>::error(msg, Some(code)))
        }
    }
}