extern crate rpl_greprs;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;
use rpl_greprs::{
    Config
};

fn main() {
    let args : Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("parse error {}", err);
        process::exit(1);
    });
    println!("Searching {} in {}", config.query, config.filename);

    if let Err(e) = rpl_greprs::run(config) {
        println!("run error {}", e);
        process::exit(1);
    }
}
