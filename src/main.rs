use std::io;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    repl::start(io::stdin(), io::stdout());
}
