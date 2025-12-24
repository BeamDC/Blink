use std::fmt::Display;
use std::ops::Range;
use regex::Regex;
use crate::compiler::CompileError;
use crate::compiler::tokenization::token::{Token, TokenKind, TokenStream};

pub mod token;

#[derive(Debug, Clone)]
pub struct Lexer<'lx> {
    source: &'lx str,
    start: usize,
    pos: usize,
    pub err_found: bool,
}

impl<'lx> Lexer<'lx> {
    pub fn new(source: &'lx str) -> Self {
        Self {
            source,
            start: 0,
            pos: 0,
            err_found: false,
        }
    }

    /// get the current position of the lexer, in terms of lines and columns
    pub fn pos(&self) -> (usize, usize){
        let line = self.source[0..self.pos]
            .chars()
            .filter(|&c| c == '\n')
            .count() + 1;

        let col = self.source[0..self.pos]
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .rposition(|&c| c == '\n')
            .map_or(self.pos, |i| self.pos - i);

        (line, col)
    }

    /// return the char at the current position in the source,
    /// and increment the current position by one
    pub fn advance(&mut self) -> Option<char> {
        let res = self.source.chars().nth(self.pos);
        self.pos += 1;
        res
    }

    /// return the current position to the new position.
    /// Cannot return to a position before the start index,
    /// as such the current position will be set to start
    fn return_to(&mut self, new_pos: usize) {
        if new_pos > self.pos { return }
        if new_pos < self.start {
            self.pos = self.start;
            return
        }
        self.pos = new_pos;
    }

    /// bump start up to the current position.
    fn consume(&mut self) {
        self.start = self.pos;
    }

    /// check if all the upcoming chars in a lexer match the specified
    /// string slice. if they do, advance the lexer past that substring,
    /// otherwise it remains unchanged.
    pub fn match_literal(&mut self, pat: &str) -> Option<Range<usize>> {
        for i in 0..pat.len() {
            if pat.chars().nth(i) != self.advance() {
                self.return_to(self.start);
                return None;
            }
        }
        let span = Some(self.start..self.pos);
        self.consume();
        span
    }

    pub fn match_regex(&mut self, pat: &str) -> Option<Range<usize>> {
        let mut pattern = String::from("^");
        pattern.push_str(pat);
        let re = Regex::new(&pattern).unwrap();
        let mat = re.find(&self.source[self.start..])?;
        let match_len = mat.end() - mat.start();
        self.pos = self.start + match_len;

        let span = Some(self.start..self.pos);
        self.consume();
        span
    }

    pub fn get_span(&self, span: &Range<usize>) -> Option<&'lx str> {
        if span.end > self.source.len() { return None }
        Some(&self.source[span.start..span.end])
    }
}

impl<'lx> Iterator for Lexer<'lx> {
    type Item = Result<Token<'lx>, CompileError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.err_found | (self.pos >= self.source.len()) { return None }
        let res = TokenKind::next_token(self);
        if res.is_err() {
            self.err_found = true;
        }
        Some(res)
    }
}

/// A trait for converting a value into a Vec of [`Token`]
///
/// this trait is not intended to me implemented manually,
/// as it should work out of the box for any type that satisfies
/// its trait bounds.
///
/// when derived, this will also give an implementation of `fmt::Display`
/// which is intended as a helper for parsing
pub trait Tokenize<'lx>: Sized + Display + Clone
where
    Self: 'lx
{
    /// generate a TokenStream from some input string,
    /// if any errors are encountered during tokenization,
    /// they will be returned instead
    #[allow(non_snake_case)]
    fn Stream(src: &'lx str) -> Result<TokenStream<'lx>, CompileError> {
        let lex = Lexer::new(src.trim());
        let mut tokens = vec![];
        for r in lex { tokens.push(r?); }
        Ok(TokenStream::from_iter(tokens.into_iter()))
    }

    fn next_token(lexer: &mut Lexer<'lx>) -> Result<Token<'lx>, CompileError>;
}
