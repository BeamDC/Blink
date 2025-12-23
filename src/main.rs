use std::time::Instant;
use crate::compiler::tokenization::token::{TokenKind, TokenStream};
use crate::compiler::tokenization::Tokenize;

mod compiler;

fn main () {
    let src = r#"1.23 >>= 3 True False hello world true false "hi " "#;

    // tokenize src
    let start = Instant::now();
    let mut stream = match TokenKind::Stream(src) {
        Ok(stream) => stream,
        Err(e) => panic!("{:?}", e),
    };
    println!("Tokenized in {:?}", start.elapsed());
    println!("{:#?}", stream);

    // parse token stream
    // let start = Instant::now();
    // println!("Parsed in {:?}", start.elapsed());
    // println!("{:?}", n);
}
