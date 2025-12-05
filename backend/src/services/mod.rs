use actix_web::{HttpResponse, HttpResponseBuilder};
use diesel::SqliteConnection;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;

pub mod user_services;

pub type DbConnection = diesel_async::pooled_connection::deadpool::Object<SyncConnectionWrapper<SqliteConnection>>;


#[derive(Debug)]
pub enum DbError {
    NotFound(String),

    DbError(diesel::result::Error),

    InvalidInput(String),

    Unexpected,
}

impl From<diesel::result::Error> for DbError {
    fn from(value: diesel::result::Error) -> Self {
        DbError::DbError(value)
    }
}

impl DbError {
    ///Return the message error and che status code for an error
    pub fn handle(&self) -> (String, String) {
        match self {
            DbError::NotFound(name) => (format!("{} not found", name), "NOT_FOUND".to_string()),
            DbError::InvalidInput(msg) => (msg.to_string(), "INVALID_INPUT".to_string()),
            DbError::Unexpected => ("Unexpected error".to_string(), "UNEXPECTED".to_string()),
            DbError::DbError(e) => {
                log::error!("{}", e);
                ("Unexpected error".to_string(), "DB_ERROR".to_string())
            }
        }
    }
    ///Return the best response type from an error
    pub fn to_http_response(&self) -> HttpResponseBuilder {
        match self {
            DbError::DbError(_) | DbError::Unexpected => HttpResponse::InternalServerError(),
            DbError::InvalidInput(_) => HttpResponse::BadRequest(),
            DbError::NotFound(_) => HttpResponse::NotFound(),
        }
    }
}
