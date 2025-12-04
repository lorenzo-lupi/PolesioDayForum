use actix_web::{HttpResponse, Responder, get, post, web::{self, Path}};
use utoipa::{OpenApi};
use uuid::Uuid;

use crate::{DbPool, models::user, routes::{api_models::{self, common::{ApiResponse, ErrorResponse}, user::{UserCreate, UserDto}}, get_conn}};



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
    crate::models::user::find_by_id(&mut conn, user_id)
    .await
    .unwrap()
    .map(
        |u| HttpResponse::Ok().json(ApiResponse::success(UserDto::from(u)))
    ).unwrap_or(
        HttpResponse::NotFound().json(ApiResponse::<()>::error("User not found".to_string(), Some("404".to_string())))
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
    pool: web::Data<DbPool>,
    user_json: web::Json<UserCreate>
) -> impl Responder {
    let user_create = user_json.into_inner();
    let mut conn = super::get_conn(pool).await;
    user::add_user(&mut conn, &user_create.username, &user_create.email)
    .await
    .map(|u| UserDto::from(u))
    .map(|dto| 
        HttpResponse::Ok().json(ApiResponse::success(dto))
    ).unwrap_or(
        HttpResponse::BadRequest().json(ApiResponse::<()>::error("Cannot insert the user".to_string(), Some("400".to_string())))
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