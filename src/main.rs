use crate::compiler::parsing::ast::AstNode;
use crate::compiler::parsing::Parse;
use crate::compiler::tokenization::Tokenize;
mod compiler;


fn main () {
    let start = std::time::Instant::now();
    let ts = "1.23 + 45.6".tokenize();
    println!("Tokenization completed in {:?}", start.elapsed());

    let mut stream = match ts {
        Ok(tokens) => {
            println!("Tokenization Successful:\n{:?}", tokens);
            tokens
        },
        Err(ce) => {
            println!("Compile Error: {:?}", ce);
            return;
        }
    };

    let start = std::time::Instant::now();
    let ast = AstNode::parse(&mut stream);
    println!("Parsing completed in {:?}", start.elapsed());

    let ast = match ast {
        Ok(ast) => {
            ast
        },
        Err(ce) => {
            println!("Compile Error: {:?}", ce);
            return;
        },
    };
}
