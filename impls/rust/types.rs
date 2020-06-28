use std::collections::HashMap;

/// Mal Types
///
/// An enum of the valid types for Mal values.
#[derive(Clone, Debug, PartialEq)]
pub enum Types {
    /// List
    List(Vec<Types>),
    Vect(Vec<Types>),
    Map(HashMap<String, Types>),

    /// Symbols
    Sym(String),

    /// Literals
    Int(isize),
    Str(String),

    /// Booleans
    Bool(bool),

    /// Bottom
    Nil,

    /// Errors
    EOF,
}

pub type ReadError = String;
pub type ReadReturn = Result<Types, ReadError>;
