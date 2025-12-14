use actix_web::{HttpResponse, get};

use crate::views::home::HomeView;

#[get("/")]
async fn home_route() -> HttpResponse {
    HttpResponse::Ok().body(dioxus_ssr::render_element(HomeView()))
}
