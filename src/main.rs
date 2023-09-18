use dotenv::dotenv;
use std::net::SocketAddr;
use toyhouse_api::auth::log_in;
use toyhouse_api::request::create_cli;
use toyhouse_api::router::get_router;

#[tokio::main]
async fn main() {
    // initialize dotenv
    dotenv().ok().unwrap();

    // initialize tracing
    tracing_subscriber::fmt::init();

    let cli = create_cli();
    let app = get_router(&cli);

    println!("LOGGING IN...");
    // get session cookie
    log_in(&cli).await;

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
