
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct PostCreateRequest {
    pub title: String,
    pub body: String,
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub body: String,
}

impl From<PostCreateRequest> for crate::models::post::Post {
    fn from(req: PostCreateRequest) -> Self {
        crate::models::post::Post {
            id: 0,
            title: req.title,
            body: req.body,
        }
    }
}

impl From<crate::models::post::Post> for PostResponse {
    fn from(post: crate::models::post::Post) -> Self {
        PostResponse {
            id: post.id,
            title: post.title,
            body: post.body,
        }
    }
}