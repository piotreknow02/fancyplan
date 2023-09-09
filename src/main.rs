mod cache;
mod spraper;
mod upgrader;

use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service()
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
