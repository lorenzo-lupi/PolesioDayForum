use utoipa::OpenApi;

pub mod posts;
pub mod api_models;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    posts::init(cfg);
}

pub fn docs() -> utoipa::openapi::OpenApi {
    posts::ApiDoc::openapi()
}