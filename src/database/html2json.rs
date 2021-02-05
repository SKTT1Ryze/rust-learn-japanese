//! Transfer HTML to Json file in this module
//! 
//! ```bash
//! cargo run h2j example.html
//! ```
//! 
//! It will pick up the japanese words from `example.html`
//! and Write into `dict.json`  

use std::{fs};
use scraper::{Html, Selector};
use super::JPWord;
use min_max::min;
#[derive(Debug)]
struct HtmlIter<'a> {
    kangji: Vec<&'a str>,
    kana: Vec<&'a str>,
    chinese: Vec<&'a str>,
    iter_index: usize,
}

impl<'a> Default for HtmlIter<'a> {
    fn default() -> Self {
        Self {
            kangji: Vec::new(),
            kana: Vec::new(),
            chinese: Vec::new(),
            iter_index: 0,
        }
    }
}

impl<'a> HtmlIter<'a> {
    pub fn add_kangji(&mut self, kangji: &'a str) {
        self.kangji.push(kangji);
    }

    pub fn add_kana(&mut self, kana: &'a str) {
        self.kana.push(kana);
    }

    pub fn add_chinese(&mut self, chinese: &'a str) {
        self.chinese.push(chinese);
    }

}

impl<'a> Iterator for HtmlIter<'a> {
    type Item = (&'a str, &'a str, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        let len = min!(self.kangji.len(), self.kana.len(), self.chinese.len());
        self.iter_index += 1;
        if self.iter_index > len {
            None
        } else {
            Some((self.kangji[self.iter_index -1 ],
                self.kana[self.iter_index - 1],
                self.chinese[self.iter_index - 1])
            )
        }
    }
}

pub fn html2json(path: &str) {
    let mut html_iter = HtmlIter::default();
    let html = fs::read_to_string(path).unwrap();
    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse("div.item-list-common").unwrap();
    for item in document.select(&selector).into_iter() {
        let texts = item.text().collect::<Vec<_>>();
        for text in texts {
            let lines: Vec<&str> = text.lines().collect();
            if lines.len() == 1 && lines[0].len() != 1 {  
                // This is Chinese
                html_iter.add_chinese(lines[0]);
            }
            else if lines.len() > 1 {
                assert!(lines.len() > 3);
                // This is kangji
                html_iter.add_kangji(lines[1]);
                // This is kana
                html_iter.add_kana(lines[2]);
            }
            else {
                // This is empty line, do nothing
            }
        }    
    }
    let jp_words: Vec<JPWord> = html_iter.into_iter().map(|h| {
        JPWord::new(h.0, h.1, h.2)
    }).collect();
    for word in jp_words {
        if let Ok(json) = serde_json::to_string(&word) {
            println!("{}", json);
        }
    }
}