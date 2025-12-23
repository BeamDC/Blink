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

    #[regex = r"[a-zA-Z_][a-zA-Z_\d]+"]
    Ident,
    #[regex = r#""[^"]*""#]
    String,
    #[regex = r#"'[^']*'"#]
    Char,
    #[regex = r"\d+\.?\d*"]
    Numeric,
    #[literal = "true"]
    True,
    #[literal = "true"]
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

#[derive(Debug, Clone)]
pub struct Token<'t, Kind: Tokenize<'t>> {
    kind: Kind,
    raw: &'t str,
    line: usize,
    col: usize,
}

impl<'t, Kind: Tokenize<'t>> fmt::Display for Token<'t, Kind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct TokenStream<'t, Kind: Tokenize<'t>> {
    stream: Vec<Token<'t, Kind>>,
    pos: usize,
}

impl<'ts, Kind: Tokenize<'ts>> TokenStream<'ts, Kind> {
    /// returns an optional reference to the next token in the stream,
    /// and advances the position in the stream
    pub fn next(&mut self) -> Option<&Token<'ts, Kind>> {
        if self.pos >= self.stream.len() { None }
        else {
            let val = &self.stream[self.pos];
            self.pos += 1;
            Some(val)
        }
    }

    /// returns an optional reference to the next item in the stream,
    /// without advancing the position in the stream.
    pub fn peek(&mut self) -> Option<&Token<'ts, Kind>> {
        if self.pos >= self.stream.len() { None }
        else {
            let val = &self.stream[self.pos];
            Some(val)
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = Token<'ts, Kind>> {
        self.stream.into_iter()
    }

    pub fn peek_kind(&mut self) -> Option<&Kind> {
        if self.pos >= self.stream.len() { None }
        else {
            let val = &self.stream[self.pos];
            Some(&val.kind)
        }
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
    pub fn from_iter<Iter: Iterator<Item = Token<'ts, Kind>>>(stream: Iter) -> Self {
        let stream = stream.collect::<Vec<_>>();
        Self {
            stream,
            pos: 0,
        }
    }
}