use std::io;
use std::io::Write;

mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let lexer = lexer::Lexer::new("let x = 5;".to_string());
    let mut parser = parser::Parser::new(lexer);
    dbg!(parser.parse());
    // repl();
}

fn repl() {
    // todo: handle 'CTRL+D'
    loop {
        print!(">> ");
        io::stdout().flush().expect("could not flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("could not read stdin");
        for token in lexer::Lexer::new(input) {
            println!("{:?}", token)
        }
    }
}
