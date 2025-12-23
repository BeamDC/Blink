use crate::compiler::CompileError;
use crate::compiler::parsing::Parse;
use crate::compiler::tokenization::token::{Token, TokenKind, TokenStream};

pub enum LiteralKind {
    Numeric,
    String,
    Char,
    Boolean,
}

pub struct Literal<'l> {
    kind: LiteralKind,
    token: Token<'l>
}

impl<'p> Parse<'p> for Literal<'p> {
    fn parse(stream: &mut TokenStream<'p>) -> Result<Self, CompileError> {
        let kind = match stream.peek_kind() {
            Some(TokenKind::Numeric) => LiteralKind::Numeric,
            Some(TokenKind::String) => LiteralKind::String,
            Some(TokenKind::Char) => LiteralKind::Char,
            Some(TokenKind::True) => LiteralKind::Boolean,
            Some(TokenKind::False) => LiteralKind::Boolean,
            t => {
                return if t.is_none() {
                    Err(CompileError::ParseError(
                        "Expected literal, found nothing".to_owned()
                    ))
                } else {
                    Err(CompileError::ParseError(
                        format!("{:?} is not a valid literal", t)
                    ))
                }
            }
        };
        Ok(Literal {
            kind,
            token: stream.next().unwrap().clone(),
        })
    }
}