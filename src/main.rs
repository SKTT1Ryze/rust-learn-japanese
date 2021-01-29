//! A Command Line Tool Written with Rust for Japanese Learning
//! Rust で作る日本語勉強ためのコマンドラインツールである

use std::env;
use command::ls::LS;

mod command;
fn main() {
    println!("Rust で作る日本語勉強ためのコマンドラインツールである");
    let args: Vec<String> = env::args().into_iter().map(|f| f.clone()).collect();
    assert!(args.len() > 1);
    match args[1].as_str() {
        "cd" => {
            panic!("CDコマンドは使いません");
        },
        "ls" => {
            assert_eq!(args.len(), 3);
            LS::ls(args[2].as_str());
        }
        _ => todo!()
    }
}
