mod lexer;

fn main() {
    let output = lexer::lex("R1 = 23.7\nR2 = 220k\n ? = R1->R2//R3");
    match output {
        Err(error) => {
            dbg!(error);
        }
        Ok(tokens) => {
            dbg!(tokens);
        }
    }
}
