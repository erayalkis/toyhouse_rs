use std::collections::HashMap;

use reqwest::Client;
use scraper::{ElementRef, Html};

pub async fn scrape<T: std::fmt::Debug>(
    cli: &Client,
    url: &str,
    html: String,
    cb: impl Fn(&Html) -> T,
) -> HashMap<i16, T> {
    let doc = scraper::Html::parse_document(&html);
    let mut store: HashMap<i16, T> = HashMap::new();
    let first_p = extract(&doc, &cb);
    store.insert(0, first_p);

    let pagination_selector = scraper::Selector::parse("ul.pagination").unwrap();
    let paginator_ele = doc.select(&pagination_selector).next();

    if paginator_ele.is_some() {
        let p = paginator_ele.unwrap();
        // First and last child are arrow buttons that lead to the prev/next page.
        let page_eles = p.children();
        let paginator_len = page_eles
            .filter_map(|c| ElementRef::wrap(c))
            .flat_map(|e| e.text())
            .collect::<Vec<_>>()
            .len();

        // We don't want to include the arrow buttons as well, so thats a -3 to the total count for the max page count.
        let max = paginator_len - 2;
        // We already have the first page, so start from the second one.
        let min = 2;

        println!("Parsing from page {} to {}", min, max);
        extract_with_pagination(cli, &mut store, url, min, max, cb).await;
    }

    return store;
}

pub fn extract<T: std::fmt::Debug>(doc: &Html, callback: impl Fn(&Html) -> T) -> T {
    let res = callback(doc);
    // println!("RETURNING {:?}", res);
    return res;
}

pub async fn extract_with_pagination<T: std::fmt::Debug>(
    cli: &Client,
    store: &mut HashMap<i16, T>,
    url: &str,
    min: i16,
    max: usize,
    callback: impl Fn(&Html) -> T,
) {
    for i in min..=max as i16 {
        println!("In page {}", i);
        let page_url = format!("{}?page={}", url, i);
        let resp = cli.get(page_url).send().await.unwrap();
        let text = resp.text().await.unwrap();

        let html = scraper::Html::parse_document(&text);
        let res = extract(&html, &callback);

        store.insert(i, res);
    }
}
