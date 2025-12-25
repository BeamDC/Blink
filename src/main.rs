use crate::compiler::parsing::expr::Expr;
use crate::compiler::parsing::Parse;
use crate::compiler::tokenization::token::TokenKind;
use crate::compiler::tokenization::Tokenize;
use std::time::Instant;

mod compiler;

fn main () {
    let src = r#"-1 + (2 << 3)"#;

    // tokenize src
    let start = Instant::now();
    let mut stream = match TokenKind::Stream(src) {
        Ok(stream) => stream,
        Err(e) => panic!("{:?}", e),
    };
    println!("Tokenized in {:?}", start.elapsed());
    println!("{:#?}", stream);

    // parse token stream
    let start = Instant::now();
    let expr = match Expr::parse(&mut stream) {
        Ok(lit) => lit,
        Err(e) => panic!("{:?}", e),
    };
    println!("Parsed in {:?}", start.elapsed());
    println!("{:#?}", expr);
}
