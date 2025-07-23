mod frontend;

use std::fs;
use crate::frontend::parser::Parser;

fn main() {
    let input = fs::read_to_string("test.txt")
        .expect("Error: couldn\'t read test.txt");

    let mut parser = Parser::new(&input);
    let program = parser.parse_program();

    println!("Parsed AST:\n{:#?}", program);
}
