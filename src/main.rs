mod server;
mod upgrader;
mod scraper;
mod cache;

use actix_web::{HttpServer, App, middleware, web::Data};
use cache::CacheProvider;
use server::{index, plan};
use std::sync::{Mutex, Arc};

const REDIS_URL: &str = "redis://redis/";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cache: Arc<Mutex<Option<CacheProvider>>> = Arc::new(Mutex::new(
        match CacheProvider::from_dsn(REDIS_URL) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(cache.clone()))
            .wrap(middleware::Logger::default())
            .service(plan)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
