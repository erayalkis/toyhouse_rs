use reqwest::{self, Client, Response};
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Serialize, Deserialize)]
struct LoginBody {
    _token: String,
    username: String,
    password: String,
}

pub async fn log_in(cli: &Client) {
    let login_url = "https://toyhou.se/~account/login";

    let username = var("TOYHOUSE_USERNAME").unwrap();
    let password = var("TOYHOUSE_PASSWORD").unwrap();

    println!(
        "LOGGING IN WITH USERNAME: {}, PASSWORD: {}",
        username, password
    );

    let csrf_res = cli.get(login_url).send().await.unwrap();
    let _token = get_csrf_token(csrf_res).await;

    let login_payload = LoginBody {
        _token,
        username,
        password,
    };
    let body = serde_urlencoded::to_string(&login_payload).unwrap();

    println!("SENDING LOGIN DATA WITH BODY {}", body);
    let login_res = cli
        .post(login_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();

    let res_body = &login_res.text().await.unwrap();
    println!("BODY: {}", res_body);
}

async fn get_csrf_token(res: Response) -> String {
    let html = res.text().await.unwrap();
    let doc = scraper::Html::parse_document(&html);

    let csrf_selector = scraper::Selector::parse("meta[name='csrf-token']").unwrap();
    let csrf_ele = doc.select(&csrf_selector).nth(0).unwrap();
    let csrf_token = csrf_ele.value().attr("content").unwrap();

    return csrf_token.to_string();
}
