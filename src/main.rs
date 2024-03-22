mod interpreter;
mod lexer;
mod parser;

use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

const HISTORY_PATH: &str = "./.iron-history";

fn main() -> Result<()> {
    println!(
        "IronCalc Version {}. Made by grqphical (https://github.com/grqphical/IronCalc)",
        env!("CARGO_PKG_VERSION")
    );
    let mut rl = DefaultEditor::new()?;
    rl.load_history(HISTORY_PATH);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(equation) => {
                rl.add_history_entry(equation.as_str())?;
                if equation.to_lowercase() == "exit" {
                    break;
                }

                let tokens = lexer::generate_tokens(equation).unwrap();

                let ast = parser::parse_expr(&tokens);
                let result = interpreter::interpret(&ast).unwrap();
                println!("{}", result);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HISTORY_PATH)?;
    return Ok(());
}
