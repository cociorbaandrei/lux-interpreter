use std::fmt;

pub enum Token {
	LeftParen(String, u32, u32),
	RightParen(String, u32, u32),
	LeftBrace(String, u32, u32),
	RightBrace(String, u32, u32),
	EndOfFile,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Token::LeftParen(lexeme, line, col) => write!(f, "{} {} {}", "LEFT_PAREN", lexeme, "null"),
			Token::RightParen(lexeme, line, col) => write!(f, "{} {} {}", "RIGHT_PAREN", lexeme, "null"),
			Token::LeftBrace(lexeme, line, col) => write!(f, "{} {} {}", "LEFT_BRACE", lexeme, "null"),
			Token::RightBrace(lexeme, line, col) => write!(f, "{} {} {}", "RIGHT_BRACE", lexeme, "null"),
			Token::EndOfFile => { write!(f, "EOF  null") }
		}
	}
}
