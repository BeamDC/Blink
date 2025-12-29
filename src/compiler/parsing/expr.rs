use crate::compiler::parsing::Parse;
use crate::compiler::tokenization::token::{Token, TokenKind, TokenStream};
use crate::compiler::CompileError;
use std::cmp::PartialOrd;
use std::fmt;
use std::ops::Deref;

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
    Call,           // ., (), [], ...
}

#[derive(Debug)]
pub enum Expr<'e> {
    /// literal values
    /// # Example:
    /// ```no_run
    /// 3.14
    /// true
    /// "hello"
    /// 'c'
    /// ```
    Literal {
        kind: LiteralKind,
        token: Token<'e>,
    },
    /// named values
    /// # Example:
    /// ```ignore
    /// x
    /// ```
    Ident(Token<'e>),
    /// infix expressions
    /// # Example:
    /// ```ignore
    /// Expr op Expr
    /// ```
    Binary{
        op: Token<'e>,
        left: Box<Expr<'e>>,
        right: Box<Expr<'e>>,
    },
    /// postfix and prefix expressions
    /// # Example:
    /// ```ignore
    /// op Expr
    /// ```
    ///
    /// OR
    ///
    /// ```ignore
    /// Expr op
    /// ```
    Unary {
        op: Token<'e>,
        operand: Box<Expr<'e>>,
    },
    /// multiple expressions contained within curly braces.
    /// # Example:
    /// ```ignore
    /// ( Expr )
    /// ```
    Group(Box<Expr<'e>>),
    /// multiple expressions contained within curly braces.
    /// # Example:
    /// ```ignore
    /// {
    ///     Expr1;
    ///     Expr2;
    ///     Expr3;
    /// }
    /// ```
    Block(Vec<Expr<'e>>),
    /// a type expression
    Type {
        /// if the variable is mutable or not
        mutable: bool,
        /// the type Identifier, will probably become an expression in the future
        ident: Token<'e>,
    },
    /// variable assignment.
    /// variables are immutable by default, but mutability can be specified
    /// # Examples:
    /// ```ignore
    /// i32 x = 2 << 3;
    /// mut usize y = 42;
    /// ```
    Assignment {
        /// the type of the variable
        var_type: Box<Expr<'e>>,
        /// the identifier associated with this variable
        ident: Token<'e>,
        /// the right side of the assignment expression
        value: Box<Expr<'e>>,
    }
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

impl<'e> fmt::Display for Expr<'e> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Literal { token, .. } => write!(f, "{}", token.raw),
            Expr::Ident(i) => write!(f, "{}", i.raw),
            Expr::Binary { op, left, right } => write!(f, "{}({}, {})", op.kind, left, right),
            Expr::Unary { op, operand } => write!(f, "{}({})", op, operand),
            Expr::Group(g) => write!(f, "({})", g),
            Expr::Block(b) => {
                write!(f, "{{")?;
                for expr in b { write!(f, "{}", expr)?; }
                write!(f, "}}")
            },
            Expr::Type { mutable, ident } => {
                if *mutable {
                    write!(f, "mut {}", ident.raw)
                } else {
                    write!(f, "{}", ident.raw)
                }
            }
            Expr::Assignment { var_type, ident, value } => {
                 write!(f, "{} {} = {} ", var_type, ident.raw, value)
            }
        }
    }
}

impl<'e> Parse<'e> for Expr<'e> {
    fn parse(stream: &mut TokenStream<'e>) -> Result<Self, CompileError> {
        Self::parse_first(stream)
    }
}

impl<'e> Expr<'e> {
    fn parse_first(stream: &mut TokenStream<'e>) -> Result<Self, CompileError> {
        // check for 'Let'
        if stream.peek_kind() == Some(&TokenKind::Let) {
            return Self::parse_let(stream);
        }

        Self::parse_precedence(stream, Precedence::Min)
    }

    /// parse a type expression from the stream
    fn parse_type_expr(stream: &mut TokenStream<'e>) -> Result<Self, CompileError> {
        // check for mut
        let mutable = if stream.peek_kind() == Some(&TokenKind::Mut) {
            stream.expect(TokenKind::Mut)?;
            true
        } else { false };

        // parse out type ident
        let ident = stream.expect(TokenKind::Ident)?;

        Ok(Expr::Type {
            mutable,
            ident,
        })
    }

    /// parse a variable assignment from the stream
    fn parse_let(stream: &mut TokenStream<'e>) -> Result<Self, CompileError> {
        // consume 'let'
        stream.expect(TokenKind::Let)?;

        // parse out variable type
        let var_type = Self::parse_type_expr(stream)?;

        let ident = stream.expect(TokenKind::Ident)?;

        // consume '='
        stream.expect(TokenKind::Assign)?;

        let value = Expr::parse_first(stream)?;

        // consume ';'
        stream.expect(TokenKind::Semicolon)?;

        Ok(Expr::Assignment {
            var_type: Box::new(var_type),
            ident,
            value: Box::new(value),
        })
    }

    const fn precendence(kind: TokenKind) -> Precedence {
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

        // todo : parse postfix expressions
        // left = Self::postfix(stream)?

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
                Ok(Expr::Ident(token))
            }
            TokenKind::Lparen => {
                let expr = Expr::parse(stream)?;
                stream.expect(TokenKind::Rparen)?;
                Ok(Expr::Group(Box::new(expr)))
            }
            k if k.is_operator() => {
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
                format!("Expected expression, found: {:?}", kind)
            ))
        }
    }
}