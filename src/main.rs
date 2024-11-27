use std::io::{self, Result, Write};

mod lexer;
mod parser;

const PROMPT: &str = "Ohm >";

fn main() -> Result<()> {
    let mut input_buffer = String::new();
    let stdin = io::stdin();

    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;
        stdin.read_line(&mut input_buffer)?;
        let lexed_input = lexer::lex(&input_buffer).unwrap();
        let parsed_input = parser::Parser::parse(lexed_input);
        match parsed_input {
            Ok(program) => {
                for ast in program {
                    dbg!(ast);
                }
            }
            Err(error) => {
                println!("ERROR: {}", error);
            }
        }

        input_buffer.clear();
    }

    Ok(())
}
