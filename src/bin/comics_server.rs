use comics::rest::*;

use {
    actix_cors::Cors,
    actix_web::{App, HttpServer},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(get_archives)
            .service(parse)
            .service(parse_methods)
            .service(get_volumes)
            .service(rename_volume)
            .service(merge_volumes)
    })
    .bind(("127.0.0.2", 8080))?
    .run()
    .await
}
