use chrono::NaiveDateTime;
use diesel::{prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn new(username: &str, email: &str) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            email: email.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
