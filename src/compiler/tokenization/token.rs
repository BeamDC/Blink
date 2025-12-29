use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;
use util::Tokenize;
use crate::compiler::tokenization::{Lexer, Tokenize};
use crate::compiler::CompileError;

#[derive(Tokenize, PartialEq, Debug, Copy, Clone)]
#[skip = r"[ \n\r\t\f]+"]
pub enum TokenKind {
    #[literal = "@"]
    At,
    #[literal = "#"]
    Pound,
    #[literal = "$"]
    Dollar,
    #[literal = "("]
    Lparen,
    #[literal = ")"]
    Rparen,
    #[literal = "{"]
    Lbrace,
    #[literal = "}"]
    Rbrace,
    #[literal = "["]
    Lbracket,
    #[literal = "]"]
    Rbracket,
    #[literal = "?"]
    Question,
    #[literal = "\\"]
    BackSlash,
    #[literal = ":"]
    Colon,
    #[literal = ";"]
    Semicolon,
    #[literal = "="]
    Assign,

    #[literal = "->"]
    Arrow,
    #[literal = "::"]
    Path,
    #[literal = "."]
    Dot,
    #[literal = ".."]
    Range,
    #[literal = "..="]
    RangeInc,

    #[literal = "+"]
    Add,
    #[literal = "+="]
    CompAdd,

    #[literal = "-"]
    Sub,
    #[literal = "-="]
    CompSub,

    #[literal = "*"]
    Mul,
    #[literal = "*="]
    CompMul,

    #[literal = "/"]
    Div,
    #[literal = "/="]
    CompDiv,

    #[literal = "%"]
    Mod,
    #[literal = "%="]
    CompMod,

    #[literal = "<<"]
    Lshift,
    #[literal = "<<="]
    CompLshift,

    #[literal = ">>"]
    Rshift,
    #[literal = ">>="]
    CompRshift,

    #[literal = "&"]
    BitAnd,
    #[literal = "&="]
    CompBitAnd,

    #[literal = "|"]
    BitOr,
    #[literal = "|="]
    CompBitOr,

    #[literal = "^"]
    BitXor,
    #[literal = "^="]
    CompBitXor,

    #[literal = "~"]
    BitNot,
    #[literal = "~="]
    CompBitNot,

    #[literal = "!"]
    Bang,
    #[literal = "!="]
    Neq,
    #[literal = "=="]
    Eq,
    #[literal = ">"]
    Gt,
    #[literal = "<"]
    Lt,
    #[literal = ">="]
    Ge,
    #[literal = "<="]
    Le,
    #[literal = "&&"]
    And,
    #[literal = "^^"]
    Xor,
    #[literal = "||"]
    Or,

    #[regex = r"[a-zA-Z_][a-zA-Z_\d]*"]
    Ident,
    #[regex = r#""[^"]*""#]
    String,
    #[regex = r#"'[^']*'"#]
    Char,
    #[regex = r"\d+(?:\.\d+)?(?:[uif](?:8|16|32|64|128))?"]
    Numeric,
    #[literal = "true"]
    True,
    #[literal = "false"]
    False,

    #[literal = "fn"]
    Fn,
    #[literal = "ret"]
    Ret,
    #[literal = "const"]
    Const,
    #[literal = "struct"]
    Struct,
    #[literal = "enum"]
    Enum,
    #[literal = "let"]
    Let,
    #[literal = "mut"]
    Mut,
    #[literal = "if"]
    If,
    #[literal = "else"]
    Else,
    #[literal = "while"]
    While,
    #[literal = "for"]
    For,
    #[literal = "loop"]
    Loop,
    #[literal = "impl"]
    Impl,
}

impl TokenKind {
    /// returns true if self can be recognized as an operator
    pub fn is_operator(&self) -> bool {
        match self {
            // mathematical
            TokenKind::Add |
            TokenKind::Sub |
            TokenKind::Mul |
            TokenKind::Div |
            TokenKind::Mod |

            // bitwise
            TokenKind::BitAnd |
            TokenKind::BitOr  |
            TokenKind::BitXor |
            TokenKind::BitNot |
            TokenKind::Rshift |
            TokenKind::Lshift |

            // comparison
            TokenKind::Bang |
            TokenKind::And  |
            TokenKind::Or   |
            TokenKind::Eq   |
            TokenKind::Lt   |
            TokenKind::Le   |
            TokenKind::Gt   |
            TokenKind::Ge   => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token<'t> {
    pub kind: TokenKind,
    pub raw: &'t str,
    pub line: usize,
    pub col: usize,
}

impl<'t> fmt::Display for Token<'t> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct TokenStream<'t> {
    pub stream: Vec<Token<'t>>,
    pub pos: usize,
}

impl<'ts> TokenStream<'ts> {
    /// parse a single [`Token`] of some given [`TokenKind`] out of the stream.
    /// If the next token is of the given type, it is returned,
    /// otherwise an error is returned.
    /// 
    /// note : this returns a parse error because checking things in the stream
    /// is considered parsing (at least by me)
    pub fn expect(&mut self, kind: TokenKind) -> Result<Token<'ts>, CompileError> {
        match self.peek_kind() {
            Some(t) if *t == kind => Ok(self.next().unwrap().clone()),
            Some(t) => Err(CompileError::ParseError(
                format!("Expected {:?}, found {:?}", kind, t),
            )),
            None => Err(CompileError::ParseError(
                format!("Expected {:?}, found end of file", kind)
            )),
        }
    }

    /// returns an optional reference to the next token in the stream,
    /// and advances the position in the stream
    pub fn next(&mut self) -> Option<&Token<'ts>> {
        if self.pos >= self.stream.len() { None }
        else {
            let val = &self.stream[self.pos];
            self.pos += 1;
            Some(val)
        }
    }

    /// returns an optional reference to the next item in the stream,
    /// without advancing the position in the stream.
    #[inline]
    pub fn peek(&mut self) -> Option<&Token<'ts>> {
        if self.pos >= self.stream.len() { None }
        else { Some(&self.stream[self.pos]) }
    }

    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = Token<'ts>> {
        self.stream.into_iter()
    }

    #[inline]
    pub fn peek_kind(&mut self) -> Option<&TokenKind> {
        if self.pos >= self.stream.len() { None }
        else { Some(&self.stream[self.pos].kind) }
    }

    #[inline]
    pub fn peek_match(&mut self, kind: TokenKind) -> bool {
        self.peek_kind() == Some(&kind)
    }

    #[inline]
    pub fn peek_any(&mut self, kinds: &[TokenKind]) -> bool {
        self.peek_kind().map_or(false, |t| kinds.contains(&t))
    }

    /// returns the current position in the stream
    #[inline(always)]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// jumps to a specific position in the stream
    ///
    /// Note: this does not check if the position is valid in the stream :P
    #[inline(always)]
    pub fn jump_to(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// construct a new [`TokenStream`] from an iterator over tokens
    #[inline(always)]
    pub fn from_iter<Iter: Iterator<Item = Token<'ts>>>(stream: Iter) -> Self {
        Self {
            stream: stream.collect::<Vec<Token<'ts>>>(),
            pos: 0,
        }
    }
}