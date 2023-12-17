use comics::rest::app_with_services;

use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(app_with_services)
        .bind(("127.0.0.2", 8080))?
        .run()
        .await
}
