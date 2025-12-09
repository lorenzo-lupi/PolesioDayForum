use actix_web::{Responder, get, post, web::{self, Path}};
use utoipa::{OpenApi};
use uuid::Uuid;

use crate::{routes::{api_models::{common::{ApiResponse, ErrorResponse}, user::{UserCreate, UserDto}}, handle_db_result}, services::{DbError, user_services::UserService}};



#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = String, Path, description = "Get users from a Uid")
    ),
    responses(
        (status = 200, description = "User found", body = ApiResponse<UserDto>),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    tag = "users"
)]
#[get("users/{id}")]
pub async fn get_user(
    user_service: web::Data<dyn UserService>,
    id: Path<String>
) -> impl Responder {
    let user_id = Uuid::try_parse(&id.into_inner());
    if user_id.is_err() {
        return handle_db_result(Err(DbError::InvalidInput("Uuid not valid".to_string())));
    }
    handle_db_result(
        user_service.into_inner().find_by_id(user_id.unwrap())
            .await
            .map(UserDto::from)
    )
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = UserCreate,
    responses(
        (status = 200, description = "User created", body = ApiResponse<UserDto>),
        (status = 400, description = "User not created", body = ErrorResponse)
    ),
    tag = "users"
)]
#[post("users")]
pub async fn add_user(
    user_service: web::Data<dyn UserService>,
    user_json: web::Json<UserCreate>
) -> impl Responder {
    let user_create = user_json.into_inner();
    handle_db_result(
        user_service.add_user(&user_create.username, &user_create.email)
        .await
        .map(UserDto::from)
    )
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_user,
        add_user
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(add_user)
       .service(get_user);
}