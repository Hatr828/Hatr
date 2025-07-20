mod frontend;

use std::fs;
use crate::frontend::lexer::Lexer;
use crate::frontend::tokens::TokenKind;
fn main() {

    let input: String = fs::read_to_string("test.txt")
        .expect("Error while opening test.txt");

    let mut lexer = Lexer::new(&input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if let TokenKind::Eof = token.kind {
            break;
        }
    }
}
