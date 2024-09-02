use std::fmt;

#[derive(Debug, PartialEq, Clone)]
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
    And(String, u32, u32),
    Class(String, u32, u32),
    Else(String, u32, u32),
    False(String, u32, u32),
    For(String, u32, u32),
    Fun(String, u32, u32),
    If(String, u32, u32),
    Nil(String, u32, u32),
    Or(String, u32, u32),
    Print(String, u32, u32),
    Return(String, u32, u32),
    Super(String, u32, u32),
    This(String, u32, u32),
    True(String, u32, u32),
    Var(String, u32, u32),
    While(String, u32, u32),
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
            Token::String(lexeme, line, col, ident) => {
                write!(f, "{} {} {}", "STRING", lexeme, ident)
            }
            Token::And(lexeme, line, col) => write!(f, "{} {} {}", "AND", lexeme, "null"),
            Token::Class(lexeme, line, col) => write!(f, "{} {} {}", "CLASS", lexeme, "null"),
            Token::Else(lexeme, line, col) => write!(f, "{} {} {}", "ELSE", lexeme, "null"),
            Token::False(lexeme, line, col) => write!(f, "{} {} {}", "FALSE", lexeme, "null"),
            Token::For(lexeme, line, col) => write!(f, "{} {} {}", "FOR", lexeme, "null"),
            Token::Fun(lexeme, line, col) => write!(f, "{} {} {}", "FUN", lexeme, "null"),
            Token::If(lexeme, line, col) => write!(f, "{} {} {}", "IF", lexeme, "null"),
            Token::Nil(lexeme, line, col) => write!(f, "{} {} {}", "NIL", lexeme, "null"),
            Token::Or(lexeme, line, col) => write!(f, "{} {} {}", "OR", lexeme, "null"),
            Token::Print(lexeme, line, col) => write!(f, "{} {} {}", "PRINT", lexeme, "null"),
            Token::Return(lexeme, line, col) => write!(f, "{} {} {}", "RETURN", lexeme, "null"),
            Token::Super(lexeme, line, col) => write!(f, "{} {} {}", "SUPER", lexeme, "null"),
            Token::This(lexeme, line, col) => write!(f, "{} {} {}", "THIS", lexeme, "null"),
            Token::True(lexeme, line, col) => write!(f, "{} {} {}", "TRUE", lexeme, "null"),
            Token::Var(lexeme, line, col) => write!(f, "{} {} {}", "VAR", lexeme, "null"),
            Token::While(lexeme, line, col) => write!(f, "{} {} {}", "WHILE", lexeme, "null"),
        }
    }
}
