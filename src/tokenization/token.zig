const tokenType = enum(u16) {
    Tilde,
    Bang,
    At,
    Pound,
    Dollar,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Question,
    BackSlash,
    Colon,
    Semicolon,
    Assign,

    Arrow,
    Path,
    Dot,
    Range,
    RangeInc,

    Add,
    CompAdd,

    Sub,
    CompSub,

    Mul,
    CompMul,

    Div,
    CompDiv,

    Mod,
    CompMod,

    Lshift,
    CompLshift,

    Rshift,
    CompRshift,

    BitAnd,
    CompBitAnd,

    BitOr,
    CompBitOr,

    BitXor,
    CompBitXor,

    BitNot,
    CompBitNot,

    Neq,
    Eq,
    Gt,
    Lt,
    Ge,
    Le,
    And,
    Xor,
    Or,

    Ident,
    String,
    Char,
    Numeric,
    True,
    False,

    Fn,
    Ret,
    Const,
    Struct,
    Enum,
    Let,
    Mut,
    If,
    Else,
    While,
    For,
    Loop,
    Impl,
};

const token = struct {
    type: tokenType,
    start: usize,
    end: usize,
    raw: []const u8,
};