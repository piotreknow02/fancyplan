use actix_web::web::Data;
use::actix_web::{get, web, Responder, HttpResponse};
use std::{
    ops::DerefMut,
    sync::{Arc, Mutex}
};
use url::Url;
use crate::{
    upgrader::Upgrader,
    scraper::get_plan,
    cache::CacheProvider
};


#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("public/index.html"))
}

#[get("/plan/{url}")]
pub async fn plan(path: web::Path<String>, data: Data<Arc<Mutex<Option<CacheProvider>>>>) -> impl Responder {
    let url = path.into_inner();
    let mut was_cached = true;

    let mut cache_mutex = data.lock().unwrap();
    let cache_provider = cache_mutex.deref_mut();
    let plan = match cache_provider {
        Some(v) => {
            match v.get(&url) {
                Some(plan) => plan,
                None => {
                    was_cached = false;
                    get_plan(url.as_str())
                        .await
                        .expect("error getting plan")
                }
            }
        }
        None => {
            was_cached = false;
            get_plan(url.as_str())
                .await
                .expect("error getting plan")
        }
    };

    let plan_url = Url::parse(&url)
        .expect("url parse error");
    let plan_domain = format!("{}://{}", plan_url.scheme(), plan_url.domain().unwrap());

    let mut upgrader = Upgrader::new(&plan, &plan_domain)
        .expect("error parsing plan");
    upgrader.default_transformations()
        .expect("error applying transformations");

    let output = upgrader.output();

    if !was_cached {
        if let Some(cache) = cache_provider {
            cache.set(&url, &output)
                .expect("error caching data")
        }    
    }
    HttpResponse::Ok().body(output)
}