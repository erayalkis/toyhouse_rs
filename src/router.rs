use axum::routing::get;
use axum::Router;

pub fn get_router() -> Router {
    axum::Router::new().route("/", get(index))
}

async fn index() -> &'static str {
    return "Hello, World!";
}
