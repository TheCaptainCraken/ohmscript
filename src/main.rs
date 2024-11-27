use std::io::{self, Write};

mod interpreter;
mod lexer;
mod parser;

const PROMPT: &str = "Ohm >";

fn main() -> Result<(), String> {
    let mut input_buffer = String::new();
    let stdin = io::stdin();

    let mut interpreter = interpreter::Interpreter::new();
    loop {
        print!("{}", PROMPT);
        io::stdout()
            .flush()
            .expect("Should always be able to flush.");
        stdin
            .read_line(&mut input_buffer)
            .expect("Should always be able to read the line.");
        let lexed_input = lexer::lex(&input_buffer)?;
        let parsed_input = parser::Parser::parse(lexed_input)?;

        interpreter.execute(parsed_input);

        input_buffer.clear();
    }
}
