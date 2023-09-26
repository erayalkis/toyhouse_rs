use actix_web::{get, web::Data, App, HttpServer};
use dotenv::dotenv;
use toyhouse_api::{auth::log_in, request::create_cli};

#[get("/character/:id/gallery")]
async fn get_character_gallery() -> &'static str {
    "You passed the auth!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let cli = create_cli();
    log_in(&cli).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(cli.clone()))
            .service(get_character_gallery)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
