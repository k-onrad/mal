extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::prelude::*;

mod printer;
mod reader;
mod types;
use printer::{pr_debug, pr_str};
use reader::read_str;

fn read() -> String {
    let mut buf = String::new();
    io::stdout().write(b"user> ").unwrap();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    buf
}

fn eval(t: types::Types) -> types::Types {
    t
}

fn print(s: String) {
    println!("{}", s)
}

fn rep() {
    print(pr_str(eval(match read_str(read()) {
        Ok(t) => t,
        Err(_) => types::Types::EOF,
    })));
}

fn main() {
    loop {
        rep();
    }
}
