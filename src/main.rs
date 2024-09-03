mod parser;
mod token;
mod tokenizer;
use core::result::Result::Ok;
use parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;
use tokenizer::Tokenizer;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <fil ename>", args[0]).unwrap();
        return ExitCode::SUCCESS;
    }

    let command = &args[1];
    let filename = &args[2];
    let mut has_err: bool = false;
    match command.as_str() {
        "tokenize" => {
            let file_contents = match fs::read_to_string(filename) {
                Ok(f) => f,
                Err(_) => todo!(),
            };
            if file_contents.is_empty() {
                println!("EOF  null");
                return ExitCode::SUCCESS;
            }
            let tokenizer = Tokenizer::new(file_contents);

            for token in tokenizer.iter() {
                match token {
                    token::Token::Error(err) => {
                        has_err = true;
                        writeln!(io::stderr(), "{}", err);
                    }
                    _ => println!("{}", token),
                }
            }
            println!("EOF  null");
        }
        "parse" => {
            let file_contents = match fs::read_to_string(filename) {
                Ok(f) => f,
                Err(_) => todo!(),
            };
            if file_contents.is_empty() {
                println!("EOF  null");
                return ExitCode::SUCCESS;
            }
            let tokenizer = Tokenizer::new(file_contents);
            let mut iter = tokenizer.iter().peekable();
            let mut parser = Parser::new(&mut iter);
            let e = parser.parse();
            match e {
                Ok(e) => println!("{}", e.pprint()),
                Err(_) => return ExitCode::from(65),
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command);
        }
    }
    if has_err {
        //	println!("err");
        return ExitCode::from(65);
    } else {
        return ExitCode::SUCCESS;
    }
}
