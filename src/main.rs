use actix_web::{get, web::Data, App, HttpServer};
use dotenv::dotenv;
use reqwest::Client;
use toyhouse_api::{
    auth::{get_authorized_users, log_in},
    request::create_cli,
};

#[get("/")]
async fn index() -> &'static str {
    "Hello, World!"
}

#[get("/character/{id}/gallery")]
async fn get_character_gallery(cli: Data<Client>) -> String {
    println!("Hit character gallery route");
    let auths = get_authorized_users(&cli).await;
    println!("{:?}", auths);

    let ser = serde_json::to_string(&auths).unwrap();
    return ser;
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
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
