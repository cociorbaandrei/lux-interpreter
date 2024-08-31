use crate::token::Token;

use anyhow::anyhow;
use anyhow::Ok;
use anyhow::Result;

pub struct Tokenizer {
	file_content: String,
}

impl Tokenizer {
	pub fn new(file_content: String) -> Result<Tokenizer> {
		if file_content.is_empty() {
			return Err(anyhow!("Input file should not be empty!"));
		}
		return Ok(Tokenizer {
			file_content: file_content,
		});
	}
	pub fn get_tokens(&self) -> Result<Vec<Token>> {
		let mut tokens: Vec<Token> = Vec::new();
		let mut line: u32 = 0;
		let mut col: u32 = 0;
		for c in self.file_content.chars() {
			match c {
				'(' => tokens.push(Token::LeftParen(c.to_string(), line, col)),
				')' => tokens.push(Token::RightParen(c.to_string(), line, col)),
				'{' => tokens.push(Token::LeftBrace(c.to_string(), line, col)),
				'}' => tokens.push(Token::RightBrace(c.to_string(), line, col)),
				'*' => tokens.push(Token::Star(c.to_string(), line, col)),
				'.' => tokens.push(Token::Dot(c.to_string(), line, col)),
				',' => tokens.push(Token::Comma(c.to_string(), line, col)),
				'+' => tokens.push(Token::Plus(c.to_string(), line, col)),
				'-' => tokens.push(Token::Minus(c.to_string(), line, col)),
				';' => tokens.push(Token::Semicolon(c.to_string(), line, col)),
				'/' => tokens.push(Token::Divide(c.to_string(), line, col)),
				' ' => {
					col += 1;
				}
				'\n' => {
					line += 1;
					col = 0;
				}
				_ => {
					println!("What");
				}
			}
			col += 1;
		}
		tokens.push(Token::EndOfFile);
		Ok(tokens)
	}
}
