use std::io;
use std::io::prelude::*;

fn read() -> String {
    let mut buf = String::new();
    io::stdout().write(b"user> ").unwrap();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    buf
}

fn eval(s: String) -> String {
    s
}

fn print(s: String) -> () {
    print!("{}", s);
}

fn rep() {
    print(eval(read()));
}

fn main() {
    loop {
        rep();
    }
}
