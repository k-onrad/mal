use lazy_static::lazy_static;
use regex::Regex;

use crate::types::{ReadError, ReadReturn, Types};

pub struct Reader {
    literals: Vec<String>,
    index: usize,
}

impl Reader {
    pub fn new(literals: Vec<String>) -> Self {
        Reader { literals, index: 0 }
    }

    pub fn consume(&mut self) -> Result<String, ReadError> {
        let s = self.peek();
        self.index += 1;
        s
    }

    pub fn peek(&self) -> Result<String, ReadError> {
        self.literals
            .get(self.index)
            .map(|s| s.to_string())
            .ok_or_else(|| reader_error("Index out of bounds!").unwrap_err())
    }
}

fn reader_error(s: &str) -> ReadReturn {
    Err(s.to_string())
}

pub fn read_str(s: String) -> ReadReturn {
    let tokens = tokenize(s);

    if tokens.len() == 0 {
        return reader_error("no input");
    }

    read_form(&mut Reader::new(tokens))
}

fn tokenize(s: String) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"##
        )
        .unwrap();
    }
    RE.find_iter(&s)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == ',' || c == '\t'
}

/// Won't consume, calls read_list or list_atom
///
/// If it finds an '(', call read_list
/// If it finds an ')', return error
fn read_form(r: &mut Reader) -> ReadReturn {
    if let Some(c) = r.peek()?.trim_matches(is_whitespace).chars().nth(0) {
        match c {
            '(' => read_list(r, ")"),
            ')' => reader_error("Unexpected ')'"),
            _ => read_atom(r),
        }
    } else {
        reader_error("Unexpected EOF")
    }
}

fn is_valid_str(s: &str) -> bool {
    lazy_static! {
        static ref STR: Regex = Regex::new(r#"(?:\\.|[^\\"])*"?"#).unwrap();
    }
    if let Some(last) = s.chars().last() {
        STR.is_match(s) && last == '"'
    } else {
        false
    }
}

fn is_valid_int(s: &str) -> bool {
    if let Ok(_) = s.parse::<isize>() {
        true
    } else {
        false
    }
}

/// Consumes token, returns Type
///
/// Gets token at current index and advances the reader
/// Matches the token to get back a MAL Type
fn read_atom(r: &mut Reader) -> ReadReturn {
    let tok = r.consume()?;
    let t = match tok.trim() {
        "true" => Types::Bool(true),
        "false" => Types::Bool(false),
        "nil" => Types::Nil,
        s if is_valid_str(s) => Types::Str(s.to_string()),
        i if is_valid_int(i) => Types::Int(i.trim_matches(is_whitespace).parse::<isize>().unwrap()),
        c => Types::Sym(c.trim_matches(is_whitespace).to_string()),
    };
    Ok(t)
}

/// Consumes Token, returns List
///
/// This should start pushing an ( into the vec
/// It then goes to the next token and read_forms it
/// When it matches a ), break out of the loop
/// Push ) into vec, return our list
fn read_list(r: &mut Reader, terminator: &str) -> ReadReturn {
    let mut list = vec![Types::Sym(r.consume()?)];

    loop {
        match r.peek() {
            Ok(t) => {
                if t.trim_matches(is_whitespace) == terminator {
                    list.push(Types::Sym(t.trim_matches(is_whitespace).to_string()));
                    break;
                }
            }
            Err(_) => return reader_error(&format!("Expected {}, got EOF", terminator)),
        };

        list.push(read_form(r)?);
    }

    let _ = r.consume();
    Ok(Types::List(list))
}
