use actix_web::{HttpResponse, Responder, get, post, web::{self, Path}};
use utoipa::OpenApi;

use crate::{models::post::Post, routes::api_models::common::ApiResponse};
use super::api_models::post::{PostResponse};

#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    params(
        ("id" = u32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Post found", body = ApiResponse<PostResponse>),
    )
)]
#[get("/posts/{id}")]
async fn get_post(id: Path<u32>) -> impl Responder {
    let post = Post {
        id: id.into_inner() as i32,
        title: "Sample Post".to_string(),
        body: "This is a sample post body.".to_string(),
    };

    let response: PostResponse = post.into();

    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    get,
    path = "/api/posts",
    responses(
        (status = 200, description = "List of all posts", body = ApiResponse<Vec<PostResponse>>)
    )
)]
#[get("/posts")]
async fn get_posts() -> impl Responder {
    let posts = vec![
        Post {
            id: 1,
            title: "First Post".to_string(),
            body: "This is the body of the first post.".to_string(),
        },
        Post {
            id: 2,
            title: "Second Post".to_string(),
            body: "This is the body of the second post.".to_string(),
        },
    ];
    let response: ApiResponse<Vec<PostResponse>> = ApiResponse::success(
        posts.into_iter()
            .map(Post::into)
            .collect()
    );
    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    post,
    path = "/api/posts",
    responses(
        (status = 201, description = "Post created successfully", body = ApiResponse<PostResponse>)
    )
)]
#[post("/posts")]
async fn create_post() -> impl Responder {
    let new_post = Post {
        id: 3,
        title: "New Post".to_string(),
        body: "This is the body of the new post.".to_string(),
    };

    let response: PostResponse = new_post.into();

    HttpResponse::Created().json(ApiResponse::success(response))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_post,
        get_posts,
        create_post
    ),
    tags(
        (name = "posts", description = "Post management endpoints")
    )
)]
pub struct ApiDoc;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts)
       .service(get_post)
       .service(create_post);
}