use anyhow::Ok;
use anyhow::{Context, Result};
use anyhow::anyhow;
use std::fmt;
use std::path::Display;
use crate::literal::Literal;

pub enum Token {
    LeftParen{lexeme : String, literal : Literal, line: u32},
    RightParen{lexeme : String, literal : Literal, line: u32}
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftParen { lexeme, literal, line } => {
                write!(f, "{} {} {}", "LEFT_PAREN", lexeme, literal)
            },
            Token::RightParen {lexeme, literal, line } => {
                write!(f, "{} {} {}", "RIGHT_PAREN", lexeme, literal)
            }
        }
        
    }
}