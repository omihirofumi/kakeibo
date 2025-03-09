use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> &'static str {
    "Hello world!"
}
