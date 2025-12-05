use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}