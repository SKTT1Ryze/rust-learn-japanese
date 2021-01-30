//! DataBase Implementation
//! このモジュールが完成させた機能は：　　
//!　+ JSON　ファイルの解析
//! + データベースの構築

use serde_derive::{Deserialize, Serialize};

pub struct JPDataBase {
    jp_dict: Vec<JPWord>    
}

#[derive(Deserialize, Serialize)]
pub struct JPWord {
    kangji: String,
    kana: String,
    chinese: String,
    sentence: Vec<String>,
}

use std::path::Path;
impl JPDataBase {
    /// Create a Japanese DataBase from specified json path
    /// TODO: example
    pub fn new<P: AsRef<Path>>(json_path: P) -> Self {
        let json = std::fs::read_to_string(json_path).expect("failed to read json file");
        let jp_dict: Vec<JPWord> = serde_json::from_str(json.as_str())
            .expect("failed to create japanese database from json");
        Self {
            jp_dict
        }
    }
}



#[test]
fn test_jpword() {
    let json = r#"
        {
            "kangji": "勉強",
            "kana": "べんきょう",
            "chinese": "学习",
            "sentence": []
        }"#;
    let jp_word: JPWord = serde_json::from_str(json).unwrap();
    assert_eq!(jp_word.kangji, String::from("勉強"));
    assert_eq!(jp_word.kana, String::from("べんきょう"));
    assert_eq!(jp_word. chinese, String::from("学习"));
    assert!(jp_word.sentence.is_empty());
}

#[test]
fn test_jpdatabase() {
    let json = r#"
        [
            {"kangji": "勉強", "kana": "べんきょう", "chinese": "学习", "sentence": []},
            {"kangji": "アニメ", "kana": "あにめ", "chinese": "动漫", "sentence": ["このアニメはすごく面白い"]}
        ]
    "#;
    let jp_database: Vec<JPWord> = serde_json::from_str(json).unwrap();
    assert_eq!(jp_database.len(), 2);
    assert_eq!(jp_database[0].kangji, "勉強");
    assert_eq!(jp_database[0].kana, "べんきょう");
    assert_eq!(jp_database[0].chinese, "学习");
    assert!(jp_database[0].sentence.is_empty());
    assert_eq!(jp_database[1].kangji, "アニメ");
    assert_eq!(jp_database[1].kana, "あにめ");
    assert_eq!(jp_database[1].chinese, "动漫");
    assert_eq!(jp_database[1].sentence.len(), 1);
    assert_eq!(jp_database[1].sentence[0], "このアニメはすごく面白い");
}

#[test]
fn test_jpdatabase_new() {
    use std::path::Path;
    let json_f = Path::new("./test.json");
    let jp_database = JPDataBase::new(json_f);
    assert_eq!(jp_database.jp_dict.len(), 3);
    assert_eq!(jp_database.jp_dict[0].kangji, "勉強");
    assert_eq!(jp_database.jp_dict[0].kana, "べんきょう");
    assert_eq!(jp_database.jp_dict[0].chinese, "学习");
    assert!(jp_database.jp_dict[0].sentence.is_empty());
    assert_eq!(jp_database.jp_dict[1].sentence.len(), 1);
    assert_eq!(jp_database.jp_dict[2].sentence.len(), 2);
    assert_eq!(jp_database.jp_dict[2].sentence[0], "もう結婚を真剣に考えてもよい年ごろだ");
    assert_eq!(jp_database.jp_dict[2].sentence[1], "真剣になればあんなやつを負かすのはなんでもない");
}