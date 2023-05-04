#![allow(unused)]

use std::{env, fs, process, error::Error};

use minigrep::{Config, run};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments. Error : {}", err);
        process::exit(-1);
    });
    if let Err(e) = run(&config) {
        println!("Application error {e}");
        process::exit(1)
    }
}


 