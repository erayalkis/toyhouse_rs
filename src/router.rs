use std::collections::HashMap;

use axum::handler::Handler;
use axum::routing::get;
use axum::Router;
use axum_macros::debug_handler;
use reqwest::Client;

use crate::auth::get_authorized_users;

pub fn get_router(cli: &Client) -> Router {
    axum::Router::new().route("/", get(index));
    axum::Router::new().route("/auths", get(auths_route(cli)))
}

async fn index() -> &'static str {
    return "Hello, World!";
}

#[debug_handler]
async fn auths_route(cli: &Client) -> HashMap<i16, Vec<std::string::String>> {
    let res = get_authorized_users(cli).await;

    return res;
}
