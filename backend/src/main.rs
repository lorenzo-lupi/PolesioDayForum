use actix_web::{ App, HttpServer, middleware::Logger, web};
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = routes::docs();
    env_logger::init();
    HttpServer::new (move || {
        App::new()
        .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
            .service(
                web::scope("/api")
                    .configure(routes::init)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
