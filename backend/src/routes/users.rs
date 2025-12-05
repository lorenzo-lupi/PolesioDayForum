use actix_web::{HttpResponse, Responder, get, post, web::{self, Path}};
use utoipa::{OpenApi};
use uuid::Uuid;

use crate::{DbPool, models::{DbError, user}, routes::{api_models::{common::{ApiResponse, ErrorResponse}, user::{UserCreate, UserDto}}, handle_db_result}};



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
    pool: web::Data<DbPool>,
    id: Path<String>
) -> impl Responder {
    let user_id = Uuid::try_parse(&id.into_inner()).unwrap();
    let mut conn = super::get_conn(pool).await;

    handle_db_result(crate::models::user::find_by_id(&mut conn, user_id).await)
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
    pool: web::Data<DbPool>,
    user_json: web::Json<UserCreate>
) -> impl Responder {
    let user_create = user_json.into_inner();
    let mut conn = super::get_conn(pool).await;
    handle_db_result(
        user::add_user(&mut conn, &user_create.username, &user_create.email)
        .await
        .map(|u| UserDto::from(u))
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