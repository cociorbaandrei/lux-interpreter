use std::fmt;

pub enum Token {
    LeftParen(String, u32, u32),
    RightParen(String, u32, u32),
    LeftBrace(String, u32, u32),
    RightBrace(String, u32, u32),
    Star(String, u32, u32),
    Dot(String, u32, u32),
    Comma(String, u32, u32),
    Plus(String, u32, u32),
    Minus(String, u32, u32),
    Semicolon(String, u32, u32),
    Slash(String, u32, u32),
    Equal(String, u32, u32),
    EqualEqual(String, u32, u32),
    Bang(String, u32, u32),
    BangEqual(String, u32, u32),
    Less(String, u32, u32),
    LessEqual(String, u32, u32),
    Greater(String, u32, u32),
    GreaterEqual(String, u32, u32),
    Colon(String, u32, u32),
    Error(String),
    Number(String, u32, u32, f64),
    Identifier(String, u32, u32, String),
    String(String, u32, u32, String),
    EndOfFile,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftParen(lexeme, line, col) => {
                write!(f, "{} {} {}", "LEFT_PAREN", lexeme, "null")
            }
            Token::RightParen(lexeme, line, col) => {
                write!(f, "{} {} {}", "RIGHT_PAREN", lexeme, "null")
            }
            Token::LeftBrace(lexeme, line, col) => {
                write!(f, "{} {} {}", "LEFT_BRACE", lexeme, "null")
            }
            Token::RightBrace(lexeme, line, col) => {
                write!(f, "{} {} {}", "RIGHT_BRACE", lexeme, "null")
            }
            Token::Star(lexeme, line, col) => write!(f, "{} {} {}", "STAR", lexeme, "null"),
            Token::Dot(lexeme, line, col) => write!(f, "{} {} {}", "DOT", lexeme, "null"),
            Token::Comma(lexeme, line, col) => write!(f, "{} {} {}", "COMMA", lexeme, "null"),
            Token::Plus(lexeme, line, col) => write!(f, "{} {} {}", "PLUS", lexeme, "null"),
            Token::Minus(lexeme, line, col) => write!(f, "{} {} {}", "MINUS", lexeme, "null"),
            Token::Semicolon(lexeme, line, col) => {
                write!(f, "{} {} {}", "SEMICOLON", lexeme, "null")
            }
            Token::Slash(lexeme, line, col) => write!(f, "{} {} {}", "SLASH", lexeme, "null"),
            Token::Number(lexeme, line, col, ident) => {
                if ident.trunc() == *ident {
                    write!(f, "{} {} {:.1}", "NUMBER", lexeme, ident)
                } else {
                    write!(f, "{} {} {}", "NUMBER", lexeme, ident)
                }
               
            }
            Token::Error(err) => write!(f, "{}", err),
            Token::EndOfFile => {
                write!(f, "EOF  null")
            }
            Token::Identifier(lexeme, line, col, ident) => {
                write!(f, "{} {} {}", "IDENTIFIER", lexeme, "null")
            }
            Token::Equal(lexeme, line, col) => write!(f, "{} {} {}", "EQUAL", lexeme, "null"),
            Token::EqualEqual(lexeme, line, col) => {
                write!(f, "{} {} {}", "EQUAL_EQUAL", lexeme, "null")
            }
            Token::Colon(lexeme, line, col) => write!(f, "{} {} {}", "COLON", lexeme, "null"),
            Token::Bang(lexeme, line, col) => write!(f, "{} {} {}", "BANG", lexeme, "null"),
            Token::BangEqual(lexeme, line, col) => {
                write!(f, "{} {} {}", "BANG_EQUAL", lexeme, "null")
            }
            Token::Less(lexeme, line, col) => write!(f, "{} {} {}", "LESS", lexeme, "null"),
            Token::LessEqual(lexeme, line, col) => {
                write!(f, "{} {} {}", "LESS_EQUAL", lexeme, "null")
            }
            Token::Greater(lexeme, line, col) => write!(f, "{} {} {}", "GREATER", lexeme, "null"),
            Token::GreaterEqual(lexeme, line, col) => {
                write!(f, "{} {} {}", "GREATER_EQUAL", lexeme, "null")
            }
            Token::String(lexeme, line, col, ident) =>  
             write!(f, "{} {} {}", "STRING", lexeme, ident),
        }
    }
}
