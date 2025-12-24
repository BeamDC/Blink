use std::cmp::{Ordering, PartialOrd};
use crate::compiler::CompileError;
use crate::compiler::parsing::Parse;
use crate::compiler::tokenization::token::{Token, TokenKind, TokenStream};

#[derive(PartialOrd, PartialEq, Debug)]
#[repr(u8)]
pub enum Precedence {
    Min = 0,
    Assign,         // =, +=, ...
    Or,             // ||
    Xor,            // ^^
    And,            // &&
    BitOr,          // |
    BitXor,         // ^
    BitAnd,         // &
    Equality,       // ==, !=
    Comparison,     // <, >, <=, >=
    Shift,          // <<, >>
    Additive,       // +, -
    Multiplicative, // *, /, %
    Unary,          // +, -, !, ...
    Call,           // ., (), []
}

#[derive(Debug)]
pub enum Expr<'e> {
    Literal {
        kind: LiteralKind,
        token: Token<'e>,
    },
    Indent(Token<'e>),
    Binary{
        op: Token<'e>,
        left: Box<Expr<'e>>,
        right: Box<Expr<'e>>,
    },
    Unary {
        op: Token<'e>,
        operand: Box<Expr<'e>>,
    },
    Grouped(Box<Expr<'e>>)
}

#[derive(Debug)]
pub enum LiteralKind {
    Numeric,
    String,
    Char,
    Boolean,
}

impl LiteralKind {
    pub fn from_token_kind(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::Numeric => Some(LiteralKind::Numeric),
            TokenKind::String => Some(LiteralKind::String),
            TokenKind::Char => Some(LiteralKind::Char),
            TokenKind::True | TokenKind::False => Some(LiteralKind::Boolean),
            _ => None
        }
    }
}
// impl<'p> Parse<'p> for Literal<'p> {
//     fn parse(stream: &mut TokenStream<'p>) -> Result<Self, CompileError> {
//         let token = match stream.next() {
//             Some(token) => token,
//             None => {
//                 return Err(CompileError::ParseError(
//                     "Expected Literal, found end of file".into()
//                 ))
//             }
//         }.clone();
//
//         let kind = match token.kind {
//             TokenKind::Numeric => LiteralKind::Numeric,
//             TokenKind::String => LiteralKind::String,
//             TokenKind::Char => LiteralKind::Char,
//             TokenKind::True => LiteralKind::Boolean,
//             TokenKind::False => LiteralKind::Boolean,
//             k => return Err(CompileError::ParseError(
//                 format!("{:?} is not a valid literal", k)
//             ))
//
//         };
//
//         Ok(Literal { kind, token })
//     }
// }

impl<'e> Parse<'e> for Expr<'e> {
    fn parse(stream: &mut TokenStream<'e>) -> Result<Self, CompileError> {
        Self::parse_precedence(stream, Precedence::Min)
    }
}

impl<'e> Expr<'e> {
    fn precendence(kind: TokenKind) -> Precedence {
        use TokenKind as TK;

        match kind {
            TK::Assign => Precedence::Assign,
            TK::Or => Precedence::Or,
            TK::Xor => Precedence::Xor,
            TK::And => Precedence::And,
            TK::BitOr => Precedence::BitOr,
            TK::BitXor => Precedence::BitXor,
            TK::BitAnd => Precedence::BitAnd,
            TK::Neq | TK::Eq => Precedence::Equality,
            TK::Lt | TK::Gt | TK::Le | TK::Ge => Precedence::Comparison,
            TK::Lshift | TK::Rshift => Precedence::Shift,
            TK::Add | TK::Sub => Precedence::Additive,
            TK::Mul | TK::Div => Precedence::Multiplicative,
            TK::Lparen | TK::Dot => Precedence::Call,
            _ => Precedence::Min,
        }
    }

    /// parse an expression at the given level of precedence
    fn parse_precedence(stream: &mut TokenStream<'e>, current_precedence: Precedence) -> Result<Self, CompileError> {
        let mut left = Self::prefix(stream)?;

        while let Some(kind) = stream.peek_kind() {
            let precedence = Self::precendence(*kind);

            if current_precedence >= precedence { break }

            left = Self::infix(stream, left)?
        }

        Ok(left)
    }

    /// parse a prefix expression out fo the stream
    fn prefix(stream: &mut TokenStream<'e>) -> Result<Self, CompileError> {
        let token = match stream.next() {
            Some(t) => t.clone(),
            None => return Err(CompileError::ParseError(
                "Expected expression, found end of file".to_string()
            ))
        };

        match token.kind {
            TokenKind::Numeric |
            TokenKind::String  |
            TokenKind::Char    |
            TokenKind::True    |
            TokenKind::False   => {
                Ok(Expr::Literal {
                    kind: LiteralKind::from_token_kind(token.kind).unwrap(),
                    token,
                })
            }
            TokenKind::Ident => {
                Ok(Expr::Indent(token))
            }
            TokenKind::Lparen => {
                let expr = Expr::parse(stream)?;
                stream.expect(TokenKind::Rparen)?;
                Ok(Expr::Grouped(Box::new(expr)))
            }
            TokenKind::Add | TokenKind::Sub => {
                let expr = Self::parse_precedence(stream, Precedence::Unary)?;
                Ok(Expr::Unary {
                    op: token,
                    operand: Box::new(expr)
                })
            }
            kind => Err(CompileError::ParseError(
                format!("Unexpected token: {:?}", kind)
            )),
        }
    }

    /// parse a postfix expression out fo the stream
    fn postfix(stream: &mut TokenStream<'e>) -> Result<Self, CompileError> {
        todo!()
    }

    /// parse an infix expression out fo the stream
    fn infix(stream: &mut TokenStream<'e>, left: Expr<'e>) -> Result<Self, CompileError> {
        let kind = match stream.peek_kind() {
            Some(k) => k,
            None => return Err(CompileError::ParseError(
                "Expected expression, found end of file".to_string()
            ))
        };

        match kind {
            k if k.is_operator() => {
                let op = stream.next().unwrap().clone();
                let precedence = Self::precendence(op.kind);
                let right = Self::parse_precedence(stream, precedence)?;

                Ok(Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right)
                })
            }
            TokenKind::Lparen => {
                todo!("come back to this when we can parse a ParamList / ArgList")
            }
            _ => Err(CompileError::ParseError(
                format!("Expected operator or function call, found: {:?}", kind)
            ))
        }
    }
}