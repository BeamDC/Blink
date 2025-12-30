use std::fmt;
use crate::compiler::CompileError;
use crate::compiler::parsing::Parse;
use crate::compiler::tokenization::token::{Token, TokenKind, TokenStream};

#[derive(Debug)]
pub struct TypeExpr<'e> {
    /// if the variable is mutable or not
    pub mutable: bool,
    /// the type Identifier, will probably become an expression in the future
    pub ident: Token<'e>,
}

impl fmt::Display for TypeExpr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mutable {
            write!(f, "mut {}", self.ident.raw)
        } else {
            write!(f, "{}", self.ident.raw)
        }
    }
}

impl<'e> Parse<'e> for TypeExpr<'e> {
    fn parse(stream: &mut TokenStream<'e>) -> Result<Self, CompileError> {
        // check for mut
        let mutable = if stream.peek_kind() == Some(&TokenKind::Mut) {
            stream.expect(TokenKind::Mut)?;
            true
        } else { false };

        // parse out type ident
        // todo : make a more robust system for parsing out types
        let ident = stream.expect(TokenKind::Ident)?;

        Ok(TypeExpr {
            mutable,
            ident,
        })
    }
}