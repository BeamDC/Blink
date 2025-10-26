use crate::compiler::CompileError;

#[derive(PartialEq, Debug)]
pub enum Token {
    At,         // @
    Pound,      // #
    Dollar,     // $
    Lparen,     // (
    Rparen,     // )
    Lbrace,     // {
    Rbrace,     // }
    Lbracket,   // [
    Rbracket,   // ]
    Question,   // ?
    BackSlash,  // \
    Colon,      // :
    Semicolon,  // ;
    Assign,     // =

    Arrow,      // ->
    Dot,        // .
    Range,      // ..
    RangeInc,   // ..=

    Add,        // +
    CompAdd,    // +=

    Sub,        // -
    CompSub,    // -=

    Mul,        // *
    CompMul,    // *=

    Div,        // /
    CompDiv,    // /=

    Mod,        // %
    CompMod,    // %=

    Lshift,     // <<
    CompLshift, // <<=

    Rshift,     // >>
    CompRshift, // >>=

    BitAnd,     // &
    CompBitAnd, // &=

    BitOr,      // |
    CompBitOr,  // |=

    BitXor,     // ^
    CompBitXor, // ^=

    BitNot,     // ~
    CompBitNot, // ~=

    Bang,       // !
    Neq,        // !=
    Eq,         // ==
    Gt,         // >
    Lt,         // <
    Ge,         // >=
    Le,         // <=
    And,        // &&
    Xor,        // ^^
    Or,         // ||

    Ident(String),
    String(String),
    Char(String),
    Numeric(String),
    Whitespace(String),
    True,
    False,

    Fn,
    Const,
    Struct,
    Enum,
    Let,
    Mut,
    If,
    While,
    For,
    Loop,
    Impl,

    End,
}

impl Token {
    pub fn str_is_kword<S: AsRef<str>>(s: S) -> bool {
        let s = s.as_ref();
        match s {
            "true"   |
            "false"  |
            "fn"     |
            "const"  |
            "struct" |
            "enum"   |
            "let"    |
            "mut"    |
            "if"     |
            "while"  |
            "for"    |
            "loop"   |
            "impl" => true,
            _ => false,
        }
    }

    pub fn str_to_kword<S: AsRef<str>>(s: &S) -> Result<Token, CompileError> {
        let s = s.as_ref();
        let kw = match s {
            "True" => Token::True,
            "false" => Token::False,
            "fn" => Token::Fn,
            "const" => Token::Const,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "let" => Token::Let,
            "mut" => Token::Mut,
            "if" => Token::If,
            "while" => Token::While,
            "for" => Token::For,
            "loop" => Token::Loop,
            "impl" => Token::Impl,
            _ => return Err(CompileError::TokenizationError(format!("{} is not a keyword", s))),
        };
        Ok(kw)
    }
}

#[derive(Debug)]
pub struct TokenStream {
    stream: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    /// returns a reference to the next token in the stream,
    /// and advances the position in the stream
    ///
    /// if the stream has no more tokens this will return [`None`] instead
    pub fn next(&mut self) -> Option<&Token> {
        if self.pos >= self.stream.len() { None }
        else {
            let val = &self.stream[self.pos];
            self.pos += 1;
            Some(val)
        }
    }

    /// returns an optional reference to the next item in the stream,
    /// without advancing the position in the stream.
    pub fn peek(&mut self) -> Option<&Token> {
        if self.pos >= self.stream.len() { None }
        else {
            let val = &self.stream[self.pos];
            Some(val)
        }
    }

    /// returns the current position in the stream
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// jumps to a specific position in the stream
    ///
    /// Note: this does not check if the position is valid in the stream :P
    pub fn return_to(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// construct a new [`TokenStream`] from a vec of [`Token`]
    pub fn new(stream: Vec<Token>) -> Self {
        Self {
            stream,
            pos: 0,
        }
    }
}