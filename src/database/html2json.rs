//! Transfer HTML to Json file in this module
//! 
//! ```bash
//! cargo run h2j example.html
//! ```
//! 
//! It will pick up the japanese words from `example.html`
//! and Write into `dict.json`  

use std::{
    fs,
    io::{prelude::*},
};
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

pub fn html2json(path: &str, dict_path: &str) {
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
    let mut jp_words: Vec<JPWord> = html_iter.into_iter().map(|h| {
        JPWord::new(h.0, h.1, h.2)
    }).collect();
    let f_string = fs::read_to_string(dict_path).unwrap();
    let mut dict: Vec<&str> = f_string.lines().collect();
    let len = dict.len();
    assert!(len >= 2);
    let last_word = dict[len - 2];
    let mut new_word = String::from(last_word);
    if len > 2 {        
        new_word.push(',');
    }
    dict[len - 2] = new_word.as_str(); 
    let mut new_json = Vec::new();
    for i in 0..jp_words.len() {
        let word = &mut jp_words[i];
        if let Ok(mut json) = serde_json::to_string(&word) {
            json.insert(0, '\t');
            if i != jp_words.len() - 1 { json.push(','); }
            new_json.push(json);
        }
    }
    for json_item in &new_json {
        dict.insert(dict.len() - 1, json_item.as_str());
    }
    let mut f = fs::File::create(dict_path).unwrap();
    for line in dict {
        // println!("{}", line);
        f.write(line.as_bytes()).unwrap();
        f.write(b"\n").unwrap();
    }
}