use std::io::{self, Result, Write};

mod interpreter;
mod lexer;
mod parser;

const PROMPT: &str = "Ohm >";

fn main() -> Result<()> {
    let mut input_buffer = String::new();
    let stdin = io::stdin();

    let mut interpreter = interpreter::Interpreter::new();
    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;
        stdin.read_line(&mut input_buffer)?;
        let lexed_input = lexer::lex(&input_buffer).unwrap();
        let parsed_input = parser::Parser::parse(lexed_input).unwrap();

        interpreter.execute(parsed_input);

        input_buffer.clear();
    }
}
