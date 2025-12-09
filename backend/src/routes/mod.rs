use actix_web::{HttpResponse, Responder};
use serde::Serialize;
use utoipa::OpenApi;

use crate::{services::{DbError}, routes::api_models::common::ApiResponse};

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