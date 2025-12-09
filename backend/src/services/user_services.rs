use async_trait::async_trait;
use diesel_async::RunQueryDsl;
use uuid::Uuid;
use diesel::{prelude::*};

use crate::{DbPool, models::user::User, schema::users::{self, id}, services::{DbConnection, DbError}};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn add_user(
        &self,
        username: &str,
        email: &str
    ) -> Result<User, DbError>;

    async fn find_by_id(
        &self,
        uid : Uuid
    ) -> Result<User, DbError>;
}


#[derive(Clone)]
pub struct DbUserService {
    pool: DbPool,
}

impl DbUserService {
    pub fn new(pool: DbPool) -> Self {
        DbUserService { pool }
    }
    
    pub async fn get_connection(&self) -> DbConnection {
        self.pool.get().await.expect("Pool, UserService")
    }
}

#[async_trait]
impl UserService for DbUserService {
    async fn add_user(
        &self,
        username: &str,
        email: &str
    ) -> Result<User, DbError> {
        if username.is_empty() || email.is_empty() {
            return Err(DbError::InvalidInput("Username or email cannot be null".to_string()));
        }

        let new_user = User::new(username, email);
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut self.get_connection().await)
            .await
            .map_err(|e| DbError::DbError(e))?;
        
        let user = self.find_by_id(Uuid::try_parse(&new_user.id).unwrap()).await;
        if user.is_err() {
            return Err(DbError::Unexpected);
        }
        // the user is always present 
        Ok(user?)
    }

    async fn find_by_id(
        &self,
        uid : Uuid
    ) -> Result<User, DbError> {
        users::table
            .filter(id.eq(uid.to_string()))
            .select(User::as_select())
            .first(&mut self.get_connection().await)
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => DbError::NotFound(format!("User(id:{})", uid)),
                e => DbError::DbError(e)
            })
    }
}