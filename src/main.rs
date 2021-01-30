//! A Command Line Tool Written with Rust for Japanese Learning
//! Rust で作る日本語勉強ためのコマンドラインツールである

use std::env;
use command::{
    ls::LS,
    cp::CP,
    mv::MV,
    rm::RM,
    exa::EXA,
};

mod command;
mod database;

fn main() {
    let args: Vec<String> = env::args().into_iter().map(|f| f.clone()).collect();
    assert!(args.len() > 1);
    let command_args: Vec<&str> = args[2..].into_iter().map(|f| f.as_str()).collect();
    match args[1].as_str() {
        "cd" => {
            panic!("CDコマンドは使いません");
        },
        "ls" => {
            LS::excute(&command_args);
        },
        "cp" => {
            CP::excute(&command_args);
        },
        "mv" => {
            MV::excute(&command_args);
        },
        "rm" => {
            RM::excute(&command_args);
        },
        "exa" => {
            EXA::excute(&command_args);
        }
        _ => panic!("unsupported command!")
    }
}
