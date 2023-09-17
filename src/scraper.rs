use scraper::Html;

pub async fn scrape<T>(url: &str, html: String, cb: impl Fn(Html) -> T) -> T {
    // let has_pagination = true;

    // if has_pagination {
    //     // extract_with_pagination(url, doc, min, max, callback)
    // } else {
    //     // extract()
    // }
    let doc = scraper::Html::parse_document(&html);
    let res = cb(doc);

    return res;
}

pub async fn extract() {}

pub async fn extract_with_pagination(
    url: String,
    doc: String,
    min: i16,
    max: i16,
    callback: impl Fn(Html),
) {
}
