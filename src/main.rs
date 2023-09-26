use actix_web::{get, web::Data, App, HttpServer, Responder};
use toyhouse_api::request::create_cli;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = create_cli();

    HttpServer::new(move || App::new().app_data(Data::new(cli.clone())))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
