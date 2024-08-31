mod literal;
mod tokenizer;
mod token;
use std::env;
use std::fs;
use std::io::{self, Write};
use anyhow::Ok;
use anyhow::{Context, Result};
use tokenizer::Tokenizer;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <fil ename>", args[0]).unwrap();
        return Ok(());
    }
    
    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename)?;
            let tokenizer = Tokenizer::new(file_contents)?;
            for token in tokenizer.get_tokens()? {
                println!("{}", token);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command)?;
            
        }
    }
    Ok(())
}
