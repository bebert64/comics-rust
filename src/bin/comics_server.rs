use comics::rest::{get_archives, parse, parse_methods};

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_archives)
            .service(parse)
            .service(parse_methods)
    })
    .bind(("127.0.0.2", 8080))?
    .run()
    .await
}
