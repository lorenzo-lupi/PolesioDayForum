use actix_web::{ App, HttpServer, middleware::Logger, web};
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod routes;

#[derive(Clone)]
struct AppConf {
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.debug").ok();
    let openapi = routes::docs();
    env_logger::init();
    
    let conf = AppConf {
        name: std::env::var("APP_NAME").expect("Name not set"),
    };

    log::info!("Name: {}", conf.name);
    HttpServer::new (move || {
        App::new()
        .wrap(Logger::default())
        .app_data(web::Data::new(conf.clone()))
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
