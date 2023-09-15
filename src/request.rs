pub fn create_cli() -> reqwest::Client {
    let cli = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    return cli;
}
