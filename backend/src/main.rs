use std::{env, sync::Arc};

use actix_web::{ App, HttpServer, middleware::Logger, web};
use diesel::SqliteConnection;
use diesel_async::{pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool}, sync_connection_wrapper::SyncConnectionWrapper};
use utoipa_swagger_ui::SwaggerUi;

use crate::{services::user_services::{DbUserService, UserService}};

mod models;
mod routes;
mod schema; 
mod services;

type DbPool = Pool<SyncConnectionWrapper<SqliteConnection>>;

#[derive(Clone)]
struct AppConf {
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let openapi = routes::docs();
    env_logger::init();
    
    let conf = AppConf {
        name: std::env::var("APP_NAME").expect("Name not set"),
    };
    let pool = initialize_db_pool().await;
    let user_service = web::Data::from(Arc::new(DbUserService::new(pool.clone())) as Arc<dyn UserService>);
    log::info!("Name: {}", conf.name);
    HttpServer::new (move || {
        App::new()
        .wrap(Logger::default())
        .app_data(web::Data::new(conf.clone()))
        .app_data(web::Data::new(pool.clone()))
        .app_data(user_service.clone())
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", openapi.clone())
        )
        .service(
            web::scope("/api")
                .configure(routes::init)
        )
    })
    .bind(("127.0.0.1", 8080))
    .expect("cannot bind on 127.0.0.1:8080")
    .run()
    .await
}


async fn initialize_db_pool() -> DbPool {
    let db_url = env::var("DATABASE_URL").expect("Env var `DATABASE_URL` not set");

    let connection_manager = AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(db_url);
    Pool::builder(connection_manager)
        .build()
        .unwrap()
}
