use std::io;

mod lexer;
mod repl;
mod token;

fn main() {
    repl::start(io::stdin(), io::stdout());
}
