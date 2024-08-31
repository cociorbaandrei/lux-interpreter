use crate::literal::Literal;
use crate::token::Token;

use anyhow::Ok;
use anyhow::Result;
use anyhow::anyhow;


pub struct Tokenizer{
    file_content : String
}

impl Tokenizer {
    pub fn new(file_content : String) -> Result<Tokenizer> {
        if file_content.is_empty() {
            return Err(anyhow!("Input file should not be empty!"));
        }
        return Ok(Tokenizer{file_content:file_content});
    }
    pub fn get_tokens(&self) -> Result<Vec<Token>> {
        let mut tokens : Vec<Token> = Vec::new();
        let mut line : u32 = 0;
        let mut col : u32 = 0;
        for c in self.file_content.chars() {
            match c {
                '(' => {
                    tokens.push(Token::LeftParen { lexeme: c.to_string(), literal: Literal::None, line: line });
                },
                ')' => {
                    tokens.push(Token::RightParen { lexeme: c.to_string(), literal: Literal::None, line: line });
                }
                '\n' => {
                    line += 1;
                    col = 0;
                }
                _ => {
                    println!("What")
                }

            }
            col += 1;
        }
        tokens.push(Token::EndOfFile);
        Ok(tokens)
    }
}
