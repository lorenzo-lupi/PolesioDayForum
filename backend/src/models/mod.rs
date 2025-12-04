use diesel::SqliteConnection;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;

pub mod post;
pub mod user;

pub type DbConnection = diesel_async::pooled_connection::deadpool::Object<SyncConnectionWrapper<SqliteConnection>>;