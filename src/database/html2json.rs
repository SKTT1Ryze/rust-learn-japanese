//! Transfer HTML to Json file in this module
//! 
//! ```bash
//! cargo run h2j example.html
//! ```
//! 
//! It will pick up the japanese words from `example.html`
//! and Write into `dict.json`  

use std::fs;
use scraper::{Html, Selector};

pub fn html2json(path: &str) {
    println!("Hello, scraper!");
    let html = fs::read_to_string(path).unwrap();
    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse("div.item-list-common").unwrap();
    for item in document.select(&selector).into_iter() {
        let texts = item.text().collect::<Vec<_>>();
        for text in texts {
            println!("{}", text);
        }    
    }
}