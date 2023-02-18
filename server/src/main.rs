use actix_web::{get, web, App, HttpServer, Responder, middleware::Logger};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet).wrap(Logger::default())
    })
    .bind(("0.0.0.0", 7890))?
    .run()
    .await
}
