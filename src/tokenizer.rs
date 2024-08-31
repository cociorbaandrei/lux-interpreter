use crate::literal::Literal;
use crate::token::Token;

use anyhow::Ok;
use anyhow::{Context, Result};
use anyhow::anyhow;
use std::fmt;
use std::path::Display;


pub struct Tokenizer{
    file_content : String
}

impl Tokenizer {
    pub fn new(file_content : String) -> Result<Tokenizer> {
        if file_content.is_empty() {
            return Err(anyhow!("Input file should not be empty!"));
        }
        return Ok(Tokenizer{file_content:"".to_owned()});
    }
    pub fn get_tokens(&self) -> Result<Vec<Token>> {
        let mut tokens : Vec<Token> = Vec::new();
        tokens.push(Token::LeftParen { lexeme: "(".to_owned(), literal: Literal::None, line: 2 });
        Ok(tokens)
    }
}
