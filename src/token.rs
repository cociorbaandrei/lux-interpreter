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
	Divide(String, u32, u32),
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
			Token::Star(lexeme, line, col) => write!(f, "{} {} {}", "STAR", lexeme, "null"),
			Token::Dot(lexeme, line, col) => write!(f, "{} {} {}", "DOT", lexeme, "null"),
			Token::Comma(lexeme, line, col) => write!(f, "{} {} {}", "COMMA", lexeme, "null"),
			Token::Plus(lexeme, line, col) => write!(f, "{} {} {}", "PLUS", lexeme, "null"),
			Token::Minus(lexeme, line, col) => write!(f, "{} {} {}", "MINUS", lexeme, "null"),
			Token::Semicolon(lexeme, line, col) => write!(f, "{} {} {}", "SEMICOLON", lexeme, "null"),
			Token::Divide(lexeme, line, col) => write!(f, "{} {} {}", "DIVIDE", lexeme, "null"),
		}
	}
}
