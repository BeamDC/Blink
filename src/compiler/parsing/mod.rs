use crate::compiler::CompileError;
use crate::compiler::tokenization::token::{TokenKind, TokenStream};
use crate::compiler::tokenization::Tokenize;

pub mod ast;
pub mod expr;

pub trait Parse<'p>: Sized {
    /// parse `self` out of the given token stream
    fn parse(stream: &mut TokenStream<'p>) -> Result<Self, CompileError>;
}