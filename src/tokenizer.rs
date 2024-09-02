use std::iter;

use crate::token::Token;

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
        let mut line: u32 = 1;
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
                '\"' => {
                    let identifier: String = iter::once(ch)
                        .chain(std::iter::from_fn(|| {
                            iter.by_ref().next_if(|c| *c != '\"' && (c.is_alphabetic() || c.is_ascii_whitespace() || c.is_alphanumeric() || c.is_ascii()))
                        }))
                        .collect::<String>();
                    if iter.peek() == Some(&'\"') {
                        iter.next();
                        let noquote = (&identifier[1..]).to_owned();
                        let s = identifier + "\"";

                        tokens.push(Token::String(s, line, col, noquote))
                    }else {
                        tokens.push(Token::Error(format!(
                            "[line {line}] Error: Unterminated string."
                        )));
                        // println!(
                        //     "[line {line}] Error: Unterminated string."
                        // );
                    }
                        
                }
                '/' => {
                    if iter.peek() == Some(&'/') {
                        while let Some(c) = iter.next() {
                            if c == '\n' {
                                line += 1;
                                break;
                            }
                        }
                    } else {
                        tokens.push(Token::Slash(ch.to_string(), line, col));
                    }
                }
                '>' => {
                    let geq: String = iter::once(ch)
                        .chain(iter.by_ref().next_if_eq(&'='))
                        .collect::<String>();
                    if geq == ">=" {
                        tokens.push(Token::GreaterEqual(geq.to_string(), line, col));
                    } else {
                        tokens.push(Token::Greater(ch.to_string(), line, col));
                    }
                }
                '<' => {
                    let leq: String = iter::once(ch)
                        .chain(iter.by_ref().next_if_eq(&'='))
                        .collect::<String>();
                    if leq == "<=" {
                        tokens.push(Token::LessEqual(leq.to_string(), line, col));
                    } else {
                        tokens.push(Token::Less(ch.to_string(), line, col));
                    }
                }
                '!' => {
                    let beq: String = iter::once(ch)
                        .chain(iter.by_ref().next_if_eq(&'='))
                        .collect::<String>();
                    if beq == "!=" {
                        tokens.push(Token::BangEqual(beq.to_string(), line, col));
                    } else {
                        tokens.push(Token::Bang(ch.to_string(), line, col));
                    }
                }
                '=' => {
                    let eq: String = iter::once(ch)
                        .chain(iter.by_ref().next_if_eq(&'='))
                        .collect::<String>();
                    if eq == "==" {
                        tokens.push(Token::EqualEqual(eq.to_string(), line, col));
                    } else {
                        tokens.push(Token::Equal(ch.to_string(), line, col));
                    }
                }
                ':' => tokens.push(Token::Colon(ch.to_string(), line, col)),
                '0'..='9' => {
                    let n = iter::once(ch) // Start with the initial digit
                        .chain(std::iter::from_fn(|| {
                            iter.by_ref().next_if(|c| (c.is_ascii_digit() || *c == '.'))
                        })) // Chain subsequent digits
                        .collect::<String>(); // Collect them into a String
                        // Parse the string into an i64
                    col += n.to_string().len() as u32;

                    // Push the number token
                    tokens.push(Token::Number(n.clone(), line, col, n.parse()?));
                }
                '_' | 'A'..='z' => {
                    let identifier: String = iter::once(ch)
                        .chain(std::iter::from_fn(|| {
                            iter.by_ref().next_if(|c| c.is_alphanumeric() || *c=='_')
                        }))
                        .collect::<String>()
                        .parse()?;
                    col += identifier.len() as u32;
                    match identifier.as_str() {
                        "and" =>  tokens.push(Token::And(identifier.clone(), line, col)),
                        "class" =>  tokens.push(Token::Class(identifier.clone(), line, col)),
                        "else" =>  tokens.push(Token::Else(identifier.clone(), line, col)),
                        "false" =>  tokens.push(Token::False(identifier.clone(), line, col)),
                        "for" =>  tokens.push(Token::For(identifier.clone(), line, col)),
                        "fun" =>  tokens.push(Token::Fun(identifier.clone(), line, col)),
                        "if" =>  tokens.push(Token::If(identifier.clone(), line, col)),
                        "nil" =>  tokens.push(Token::Nil(identifier.clone(), line, col)),
                        "or" =>  tokens.push(Token::Or(identifier.clone(), line, col)),
                        "print" =>  tokens.push(Token::Print(identifier.clone(), line, col)),
                        "return" =>  tokens.push(Token::Return(identifier.clone(), line, col)),
                        "super" =>  tokens.push(Token::Super(identifier.clone(), line, col)),
                        "this" =>  tokens.push(Token::This(identifier.clone(), line, col)),
                        "true" =>  tokens.push(Token::True(identifier.clone(), line, col)),
                        "var" =>  tokens.push(Token::Var(identifier.clone(), line, col)),
                        "while" =>  tokens.push(Token::While(identifier.clone(), line, col)),
                        _ => {
                            tokens.push(Token::Identifier(identifier.clone(), line, col, identifier));
                        }
                    }
                    
                }
                ' ' => {
                    col += 1;
                }
                '\t' => {
                    col += 1;
                }
                '\n' => {
                    line += 1;
                    col = 0;
                }
                '\r' => {}
                _ => {
                    tokens.push(Token::Error(format!(
                        "[line {line}] Error: Unexpected character: {}",
                        ch
                    )));
                    // println!(
                    //     "[line 1] Error: Unexpected character: {}",
                    //     ch.escape_debug()
                    // );
                }
            }
        }

        tokens.push(Token::EndOfFile);
        Ok(tokens)
    }
}
