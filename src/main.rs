//! A Command Line Tool Written with Rust for Japanese Learning
//! Rust で作る日本語勉強ためのコマンドラインツールである

mod command;
mod database;

use std::env;
use std::path::Path;
use command::{
    ls::LS,
    cp::CP,
    mv::MV,
    rm::RM,
    exa::EXA,
    clear::Clear,
    cargo::Cargo,
    touch::Touch,
    ssh::SSH,
    git::Git,
    just::Just,
    rustup::RustUp,
    vim::Vim,
};
use database::JPDataBase;

fn main() {
    // Prepare Japanese DataBase
    let rlj_home = if let Ok(val) = std::env::var("RLJ_HOME") {
        val
    } else { String::from("./") };
    let rlj_home = Path::new(rlj_home.as_str());
    let json_f = rlj_home.join("dict.json");
    let jp_database = JPDataBase::new(json_f);
    let args: Vec<String> = env::args().into_iter().map(|f| f.clone()).collect();
    assert!(args.len() > 1);
    let command_args: Vec<&str> = args[2..].into_iter().map(|f| f.as_str()).collect();
    match args[1].as_str() {
        "cd" => {
            panic!("CDコマンドは使いません");
        },
        "ls" => {
            is_time_to_memorize_japanese_word(&jp_database);
            LS::excute(&command_args);
        },
        "cp" => {
            is_time_to_memorize_japanese_word(&jp_database);
            CP::excute(&command_args);
        },
        "mv" => {
            is_time_to_memorize_japanese_word(&jp_database);
            MV::excute(&command_args);
        },
        "rm" => {
            is_time_to_memorize_japanese_word(&jp_database);
            RM::excute(&command_args);
        },
        "exa" => {
            is_time_to_memorize_japanese_word(&jp_database);
            EXA::excute(&command_args);
        },
        "clear" => {
            is_time_to_memorize_japanese_word(&jp_database);
            std::thread::sleep(std::time::Duration::from_secs(3));
            Clear::excute(&command_args);
        },
        "cargo" => {
            is_time_to_memorize_japanese_word(&jp_database);
            Cargo::excute(&command_args);
        },
        "touch" => {
            is_time_to_memorize_japanese_word(&jp_database);
            Touch::excute(&command_args);
        },
        "ssh" => {
            is_time_to_memorize_japanese_word(&jp_database);
            SSH::excute(&command_args);
        },
        "git" => {
            is_time_to_memorize_japanese_word(&jp_database);
            Git::excute(&command_args);
        },
        "just" => {
            is_time_to_memorize_japanese_word(&jp_database);
            Just::excute(&command_args);
        },
        "rustup" => {
            is_time_to_memorize_japanese_word(&jp_database);
            RustUp::excute(&command_args);
        },
        "vim" => {
            is_time_to_memorize_japanese_word(&jp_database);
            Vim::excute(&command_args);
        },
        _ => panic!("unsupported command!")
    }
}

fn is_time_to_memorize_japanese_word(jp_database: &JPDataBase) {
    // Get random japanese word
    use ansi_rgb::{ Foreground, red, blue, green};
    let jpword = jp_database.rand_jpword();
    let (kangji, kana, chinese, sentenses) = jpword.value();
    println!("{}", kangji.fg(red()));
    // TODO: Handle user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read user input");
    println!("{} {}", kana.fg(blue()), chinese.fg(blue()));
    for sentense in sentenses {
        println!("{}", sentense.fg(green()));
    }
}