use actix_web::{HttpResponse, get};

use crate::views::home::HomeView;

#[get("/")]
async fn home_route() -> HttpResponse {
    HttpResponse::Ok().body(format!("<!doctype html>\n{}",dioxus_ssr::render_element(HomeView())) )
}
