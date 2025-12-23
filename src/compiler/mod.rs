use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod tokenization;
pub mod parsing;

#[derive(Debug, Clone)]
pub enum CompileError {
    TokenizationError(String),
    ParseError(String),
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            CompileError::TokenizationError(string) => string,
            CompileError::ParseError(string) => string,
        };
        write!(f, "{}", string)
    }
}

impl Error for CompileError {}