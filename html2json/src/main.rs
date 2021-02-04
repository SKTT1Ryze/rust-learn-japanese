use std::fs;
use scraper::{Html, Selector};
fn main() {
    println!("Hello, scraper!");
    let html = fs::read_to_string("test.html").unwrap();
    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse("div.item-list-common").unwrap();
    for item in document.select(&selector).into_iter() {
        let texts = item.text().collect::<Vec<_>>();
        for text in texts {
            println!("{}", text);
        }    
    }
}
