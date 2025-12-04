use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::user::User;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String
}

impl From<UserCreate> for crate::models::user::User {
    fn from(value: UserCreate) -> Self {
        crate::models::user::User::new(&value.username, &value.email)
    }
}

impl From<crate::models::user::User> for UserDto {
    fn from(value: crate::models::user::User) -> Self {
        UserDto { 
            id: value.id,
            username: value.username, 
            email: value.email
        }
    }
}

