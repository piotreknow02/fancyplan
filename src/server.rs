use::actix_web::{get, web, Responder, HttpResponse};
use url::Url;
use crate::{
    upgrader::Upgrader,
    scraper::get_plan,
};


#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("public/index.html"))
}

#[get("/plan/{url}")]
pub async fn plan(path: web::Path<String>) -> impl Responder {
    let url = path.into_inner();
    let plan = get_plan(url.as_str())
        .await
        .expect("error getting plan");
    let plan_url = Url::parse(&url)
        .expect("url parse error");
    let plan_domain = plan_url.scheme().clone().to_owned() + "://"+ plan_url.domain().unwrap();
    let mut upgrader = Upgrader::new(&plan, &plan_domain)
        .expect("error parsing plan");
    upgrader.default_transformations()
        .expect("error applying transformations");
    HttpResponse::Ok().body(upgrader.output())
}