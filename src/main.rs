mod server;
mod upgrader;
mod scraper;
mod cache;

use actix_web::{HttpServer, App, middleware, web::Data};
use cache::CachePrivoder;
use server::{index, plan};

const REDIS_URL: &str = "redis://redis/";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cache: Option<CachePrivoder> = match CachePrivoder::from_dsn(REDIS_URL) {
        Ok(v) => Some(v),
        Err(e) => None,
    };

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(plan)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
