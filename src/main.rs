use std::fs;

mod lexer;
use lexer::{Token, Tokenizer};

mod parser;
use parser::Parser;


const FILENAME: &str = "file.txt";


fn main() -> std::io::Result<()> {

    let s: String = fs::read_to_string(FILENAME)
        .expect("Failed to open file");

    let mut tokenizer = Tokenizer::new(s);
    let tokens: Vec<Token> = tokenizer.tokenize();
    println!("{:#?}", tokens);

    let mut parser = Parser::new(tokens);
    parser.parse();



    Ok(())

}
