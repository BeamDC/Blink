use std::fmt;
use std::fmt::Formatter;
use crate::compiler::CompileError;
use crate::compiler::parsing::expr::Expr;
use crate::compiler::parsing::Parse;
use crate::compiler::parsing::type_expr::TypeExpr;
use crate::compiler::tokenization::token::{Token, TokenKind, TokenStream};

// todo : Add implementations
pub enum Statement<'s> {
    Import,
    Const,
    /// variable assignment.
    /// variables are immutable by default, but mutability can be specified
    /// # Examples:
    /// ```ignore
    /// let i32 x = 2 << 3;
    /// let mut usize y = 42;
    /// ```
    Let {
        /// the type of the variable
        var_type: TypeExpr<'s>,
        /// the identifier associated with this variable
        ident: Token<'s>,
        /// the right side of the assignment expression
        value: Box<Expr<'s>>,
    },
    FuncDef,
    Struct,
    Enum,
    TraitDef,
    Impl,
}

impl fmt::Display for Statement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Import => todo!(),
            Statement::Const => todo!(),
            Statement::Let { var_type, ident, value } => {
                write!(f, "{} {} = {} ", var_type, ident, value)
            },
            Statement::FuncDef => todo!(),
            Statement::Struct => todo!(),
            Statement::Enum => todo!(),
            Statement::TraitDef => todo!(),
            Statement::Impl => todo!(),
        }
    }
}

impl<'s> Parse<'s> for Statement<'s> {
    fn parse(stream: &mut TokenStream<'s>) -> Result<Self, CompileError> {
        match stream.peek_kind() {
            Some(&TokenKind::Let) => Self::parse_let(stream),
            _ => { todo!() }
        }
    }
}

impl<'s> Statement<'s> {
    fn parse_let(stream: &mut TokenStream<'s>) -> Result<Self, CompileError> {
        // consume 'let'
        stream.expect(TokenKind::Let)?;

        // parse out variable type
        let var_type = TypeExpr::parse(stream)?;

        let ident = stream.expect(TokenKind::Ident)?;

        // consume '='
        stream.expect(TokenKind::Assign)?;

        let value = Expr::parse_first(stream)?;

        // consume ';'
        stream.expect(TokenKind::Semicolon)?;

        Ok(Statement::Let {
            var_type,
            ident,
            value: Box::new(value),
        })
    }
}

