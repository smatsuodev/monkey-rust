use crate::lexer::Lexer;
use crate::token::TokenKind;
use std::io::{BufRead, BufReader, Read, Write};

const PROMPT: &'static str = ">> ";

pub fn start(buf_in: impl Read, mut buf_out: impl Write) {
    let mut reader = BufReader::new(buf_in);

    loop {
        buf_out
            .write(PROMPT.as_bytes())
            .and_then(|_| buf_out.flush())
            .expect("failed to write prompt");

        let mut line = String::new();
        reader.read_line(&mut line).expect("failed to read line");

        let mut lexer = Lexer::new(&line);

        while let token = lexer.next_token() {
            if token.kind == TokenKind::EOF {
                break;
            }

            buf_out
                .write_fmt(format_args!("{:?}\n", token))
                .expect("failed to write output");
        }
    }
}
