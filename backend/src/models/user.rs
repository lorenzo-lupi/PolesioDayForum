use chrono::NaiveDateTime;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use diesel::{prelude::*};
use uuid::Uuid;
use crate::{models::DbConnection, schema::{users::id, *}};

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

pub async fn add_user(conn: &mut DbConnection,
            username: &str,
            email: &str) -> Result<User, String> {
    let new_user = User::new(username, email);
    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .await
        .map_err(|e| format!("Cannot add {}", &new_user.username.to_string()))?;
    
    let user = find_by_id(conn, Uuid::try_parse(&new_user.id).unwrap()).await;
    // the user is always present 
    Ok(user
        .unwrap()
        .unwrap()
    )
}

pub async fn find_by_id(conn: &mut DbConnection,
    uid : Uuid
) -> Result<Option<User>, String> {
    Ok(
        users::table
            .filter(id.eq(uid.to_string()))
            .select(User::as_select())
            .first(conn)
            .await
            .optional()
            .map_err(|e| format!("Error fatching user {}", uid))?
    )
}