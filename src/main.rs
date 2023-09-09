mod server;
mod upgrader;
mod scraper;
mod cache;

use actix_web::{HttpServer, App};
use server::{index, plan};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(plan)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
