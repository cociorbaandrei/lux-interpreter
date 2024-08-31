use std::array::from_fn;
use std::borrow::Borrow;
use std::iter;

use crate::token::Token;

use anyhow::anyhow;
use anyhow::Ok;
use anyhow::Result;

pub struct Tokenizer {
    file_content: String,
}

impl Tokenizer {
    pub fn new(file_content: String) -> Tokenizer {
        return Tokenizer {
            file_content: file_content,
        };
    }
    pub fn get_tokens(&self) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut iter = self.file_content.chars().peekable();
        let mut line: u32 = 0;
        let mut col: u32 = 0;
        while let Some(ch) = iter.next() {
            match ch {
                '(' => tokens.push(Token::LeftParen(ch.to_string(), line, col)),
                ')' => tokens.push(Token::RightParen(ch.to_string(), line, col)),
                '{' => tokens.push(Token::LeftBrace(ch.to_string(), line, col)),
                '}' => tokens.push(Token::RightBrace(ch.to_string(), line, col)),
                '*' => tokens.push(Token::Star(ch.to_string(), line, col)),
                '.' => tokens.push(Token::Dot(ch.to_string(), line, col)),
                ',' => tokens.push(Token::Comma(ch.to_string(), line, col)),
                '+' => tokens.push(Token::Plus(ch.to_string(), line, col)),
                '-' => tokens.push(Token::Minus(ch.to_string(), line, col)),
                ';' => tokens.push(Token::Semicolon(ch.to_string(), line, col)),
                '/' => tokens.push(Token::Divide(ch.to_string(), line, col)),
                '=' => tokens.push(Token::Equals(ch.to_string(), line, col)),
                ':' => tokens.push(Token::Colon(ch.to_string(), line, col)),
                '1'..='9' => {
                    let n: f64 = iter::once(ch) // Start with the initial digit
                        .chain(std::iter::from_fn(|| {
                            iter.by_ref().next_if(|c| c.is_ascii_digit() || *c == '.')
                        })) // Chain subsequent digits
                        .collect::<String>() // Collect them into a String
                        .parse()?; // Parse the string into an i64
                    col += n.to_string().len() as u32;

                    // Push the number token
                    tokens.push(Token::Number(n.to_string(), line, col, n));
                }
                'A'..='z' => {
                    let identifier: String = iter::once(ch)
                        .chain(std::iter::from_fn(|| {
                            iter.by_ref().next_if(|c| c.is_alphabetic())
                        }))
                        .collect::<String>()
                        .parse()?;
                    col += identifier.len() as u32;
                    tokens.push(Token::Identifier(identifier.clone(), line, col, identifier));
                }
                ' ' => {
                    col += 1;
                }
                '\n' => {
                    line += 1;
                    col = 0;
                }
                _ => {
                    tokens.push(Token::Error(format!(
                        "[line 1] Error: Unexpected character: {}",
                        ch
                    )));
                    //writeln!("[line 1] Error: Unexpected character: {}", c);
                }
            }
        }

        tokens.push(Token::EndOfFile);
        Ok(tokens)
    }
}
