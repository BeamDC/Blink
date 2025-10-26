use std::str::Chars;
use crate::compiler::tokenization::token::{Token, TokenStream};
use crate::compiler::CompileError;
use std::iter::Peekable;

pub(crate) mod token;

/// Consumes an iterator over chars while the current char matches the provided pattern.
///
/// Returns the string that was accumulated during this process.
macro_rules! consume_while {
    ($c:expr, $chars: expr, $pattern:pat $(if $guard:expr)? $(,)?) => ({
        let mut acc = String::new();
        acc.push($c);
        while let Some(&next) = $chars.peek() {
            match next {
                $pattern $(if $guard)? => acc.push($chars.next().unwrap()),
                _ => break,
            }
        }
        acc
    });
    ($chars: expr, $pattern:pat $(if $guard:expr)? $(,)?) => ({
        let mut acc = String::new();
        while let Some(&next) = $chars.peek() {
            match next {
                $pattern $(if $guard)? => acc.push($chars.next().unwrap()),
                _ => break,
            }
        }
        acc
    });
}

/// Consumes an iterator over chars while the current char does not match the provided pattern.
///
/// Returns the string that was accumulated during this process.
macro_rules! consume_while_not {
    ($c:expr, $chars: expr, $pattern:pat $(if $guard:expr)? $(,)?) => ({
        let mut acc = String::new();
        acc.push($c);
        while let Some(&next) = $chars.peek() {
            match next {
                $pattern $(if $guard)? => break,
                _ => acc.push($chars.next().unwrap()),
            }
        }
        acc
    });
    ($chars: expr, $pattern:pat $(if $guard:expr)? $(,)?) => ({
        let mut acc = String::new();
        while let Some(&next) = $chars.peek() {
            match next {
                $pattern $(if $guard)? => break,
                _ => acc.push($chars.next().unwrap()),
            }
        }
        acc
    });
}

/// given a peekable iterator over some chars, check to see if any of the provided
/// patterns can be matched to the next values of the iterator.
///
/// returns the first match found, or some default fallback token if no matches are found.
///
/// this also expects that the first char in the pattern has been consumed,
/// in the future me may revisit this to make it more generalized.
macro_rules! match_op {
    ($chars:expr, $first:literal => $first_token:expr
    $(,$pattern:literal => $token:expr)* ; $default:expr) => {{
        if Self::long_match($first, $chars) {
            $first_token
        }
        $(
            else if Self::long_match($pattern, $chars) {
                $token
            }
        )*
        else { $default }
    }};
}

/// A trait for converting a value into a Vec of [`Token`]
///
/// this trait is not intended to me implemented manually,
/// as it should work out of the box for any type that satisfies
/// its trait bounds
pub trait Tokenize: ToString {

    fn peek_nth(chars: &Peekable<Chars>, idx: usize) -> Option<char> {
        chars.clone().nth(idx)
    }

    fn long_match<C: ToString>(expected: C, chars: &mut Peekable<Chars>) -> bool {
        for (i, c) in expected.to_string().chars().skip(1).enumerate() {
            match Self::peek_nth(chars, i) {
                Some(peeked) if peeked == c => continue,
                _ => return false,
            }
        }
        for _ in 0..expected.to_string().len() {
            _ = chars.next();
        }
        true
    }

    fn operator(first: char, chars: &mut Peekable<Chars>) -> Result<Token, CompileError> {
        // todo : match op requires that the patterns are sorted from
        //   longest to shortest in order to maintain the algorithms greedy
        //   nature, maybe we want to change it somehow to allow for arbitrary
        //   ordering, which is then sorted before generation.
        let res = match first {
            '.' => match_op!(chars,
                "..=" => Token::RangeInc,
                ".." => Token::Range;
                Token::Dot
            ),
            '+' => match_op!(chars,
                "+=" => Token::CompAdd;
                Token::Add
            ),
            '-' => match_op!(chars,
                "-=" => Token::CompSub,
                "->" => Token::Arrow;
                Token::Sub
            ),
            '*' => match_op!(chars,
                "*=" => Token::CompMul;
                Token::Mul
            ),
            '/' => match_op!(chars,
                "/=" => Token::CompDiv;
                Token::Div
            ),
            '%' => match_op!(chars,
                "%=" => Token::CompMod;
                Token::Mod
            ),
            '<' => match_op!(chars,
                "<<=" => Token::CompLshift,
                "<<" => Token::Lshift,
                "<=" => Token::Le;
                Token::Lt
            ),
            '>' => match_op!(chars,
                ">>=" => Token::CompRshift,
                ">>" => Token::Rshift,
                ">=" => Token::Ge;
                Token::Gt
            ),
            '&' => match_op!(chars,
                "&=" => Token::CompBitAnd,
                "&&" => Token::And;
                Token::BitAnd
            ),
            '|' => match_op!(chars,
                "|=" => Token::CompBitOr,
                "||" => Token::Or;
                Token::BitOr
            ),
            '^' => match_op!(chars,
                "^=" => Token::CompBitXor,
                "^^" => Token::Xor;
                Token::BitXor
            ),
            '~' => match_op!(chars,
                "~=" => Token::CompBitNot;
                Token::BitNot
            ),
            '!' => match_op!(chars,
                "!=" => Token::Neq;
                Token::Bang
            ),
            '=' => match_op!(chars,
                "==" => Token::Eq;
                Token::Assign
            ),
            _ => return Err(CompileError::TokenizationError(format!("Unexpected Token in operator {:?}", first)))
        };
        Ok(res)
    }

    /// build and return the next token
    fn next_token(chars: &mut Peekable<Chars>) -> Result<Token, CompileError> {
        // todo : pre processing step, remove all comments '// .. \n'
        let Some(c) = chars.next() else { return Ok(Token::End) };
        match c {
            '@' => Ok(Token::At),
            '#' => Ok(Token::Pound),
            '$' => Ok(Token::Dollar),
            '(' => Ok(Token::Lparen),
            ')' => Ok(Token::Rparen),
            '{' => Ok(Token::Lbrace),
            '}' => Ok(Token::Rbrace),
            '[' => Ok(Token::Lbracket),
            ']' => Ok(Token::Rbracket),
            '?' => Ok(Token::Question),
            '\\' => Ok(Token::BackSlash),
            ':' => Ok(Token::Colon),
            ';' => Ok(Token::Semicolon),

            // numbers
            '0'..='9' => {
                let mut acc = consume_while!(c, chars, '0'..='9');
                if chars.peek() == Some(&'.') {
                    // unwrap is safe since we know that next will return '.'
                    acc.push(chars.next().unwrap());
                    acc.push_str(&consume_while!(chars, '0'..='9'));
                }
                Ok(Token::Numeric(acc))
            }

            // identifiers
            'a'..='z' | 'A'..='Z' | '_' => {
                let s = consume_while!(c, chars, 'a'..='z' | 'A'..='Z' | '_');
                match Token::str_is_kword(&s) {
                    true => Ok(Token::str_to_kword(&s)?),
                    false => Ok(Token::Ident(s)),
                }
            }

            // whitespace
            ' ' | '\n' | '\t' | '\r' => {
                Ok(Token::Whitespace(consume_while!(c, chars, ' ' | '\n' | '\t' | '\r')))
            }

            // todo : operators
            '.' |
            '+' |
            '-' |
            '*' |
            '/' |
            '%' |
            '<' |
            '>' |
            '&' |
            '|' |
            '^' |
            '~' |
            '!' |
            '=' => Self::operator(c, chars),

            '\'' => {
                Ok(Token::Char(consume_while_not!(chars, '\'')))
            }

            '"' => {
                Ok(Token::String(consume_while_not!(chars, '"')))
            }

            e => Err(CompileError::TokenizationError(format!("Unexpected Char: {:?}", e)))
        }
    }

    /// build and return a token stream from some value
    fn tokenize(&self) -> Result<TokenStream, CompileError> {
        let mut tokens = vec![];
        let string = self.to_string();
        let mut chars = string.chars().peekable();

        while tokens.last() != Some(&Token::End) {
            tokens.push(Self::next_token(&mut chars)?);
        }

        Ok(TokenStream::new(tokens))
    }
}

impl Tokenize for String {}
impl Tokenize for &str {}