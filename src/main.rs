mod functions;
mod interpreter;
mod lexer;
mod parser;

use std::collections::HashMap;

use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

const HISTORY_PATH: &str = "./.argon-history";

fn repl() {
    println!(
        "Argon Version {}. Made by grqphical (https://github.com/grqphical/Argon). Type 'exit' to exit.",
        env!("CARGO_PKG_VERSION")
    );
    let mut rl = DefaultEditor::new().unwrap();
    rl.load_history(HISTORY_PATH);

    let mut variables: HashMap<String, f64> = HashMap::new();
    let mut functions = functions::load_functions();

    loop {
        let readline = rl.readline("(argon)>> ");
        match readline {
            Ok(equation) => {
                rl.add_history_entry(equation.as_str());
                if equation.to_lowercase() == "exit" {
                    break;
                }

                let tokens = match lexer::generate_tokens(equation) {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        continue;
                    }
                };

                let ast = match parser::parse_expr(&tokens, &mut variables) {
                    Ok(ast) => ast,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        continue;
                    }
                };
                let result = match interpreter::interpret(&ast, &mut variables, &mut functions) {
                    Ok(result) => result,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        continue;
                    }
                };
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
            }
        }
    }
    rl.save_history(HISTORY_PATH).unwrap();
}

fn run_file(file: &str) -> Result<()> {
    // Execute a file if one was provided
    let file = std::fs::read_to_string(file)?;
    let mut variables: HashMap<String, f64> = HashMap::new();
    let mut functions = functions::load_functions();

    for line in file.lines() {
        let tokens = lexer::generate_tokens(line.to_string())?;
        let ast = parser::parse_expr(&tokens, &mut variables)?;
        let result = interpreter::interpret(&ast, &mut variables, &mut functions)?;
        println!("{}", result);
    }

    return Ok(());
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        match run_file(&args[1]) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    } else {
        repl();
    }
}
