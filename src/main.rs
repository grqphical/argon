mod functions;
mod interpreter;
mod lexer;
mod parser;

use std::collections::HashMap;

use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

const HISTORY_PATH: &str = "./.iron-history";

fn main() -> Result<()> {
    println!(
        "Argon Version {}. Made by grqphical (https://github.com/grqphical/IronCalc). Type 'exit' to exit.",
        env!("CARGO_PKG_VERSION")
    );
    let mut rl = DefaultEditor::new()?;
    rl.load_history(HISTORY_PATH);

    let mut variables: HashMap<String, f64> = HashMap::new();
    let mut functions = functions::load_functions();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(equation) => {
                rl.add_history_entry(equation.as_str())?;
                if equation.to_lowercase() == "exit" {
                    break;
                }

                let tokens = lexer::generate_tokens(equation).unwrap();

                let ast = parser::parse_expr(&tokens, &mut variables).unwrap();
                let result = interpreter::interpret(&ast, &mut variables, &mut functions).unwrap();
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
