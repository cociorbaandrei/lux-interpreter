use std::iter::Peekable;

use crate::parser;
use crate::tokenizer::Tokenizer;
use crate::{token::Token, tokenizer::TokenIter};
use anyhow::anyhow;
use anyhow::Ok;
use anyhow::Result;
use anyhow::Error;
#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
    Power,
    Negative,
}

#[derive(Debug, PartialEq)]
pub enum Expression{
    Binary(Operator, Box<Expression>, Box<Expression>),
    Unary(Operator, Box<Expression>),
    Number(f64)
}

struct Parser<'a> {
    iter: &'a mut Peekable<TokenIter<'a>>,
}
impl<'a> Parser<'a> {
    fn new(iter: &'a mut Peekable<TokenIter<'a>>) -> Self {
        Parser { iter: iter }
    }
    fn assert_next(&mut self, token: Token) -> Result<()> {
        let next = self.iter.next();
        if let Node = next {
            return Err(anyhow!("Undexpected end of input."))
        }
        if next.as_ref() != Some(&token) {
            return  Err(anyhow!("Expected {:?} actual {:?}", token, next.as_ref()));
        }
        Ok(())
    }
    fn primary(&mut self) -> Result<Expression> {
        let next = self.iter.next().unwrap();
        match next {
            Token::Number(lexeme, line, col, ident) => Ok(Expression::Number((ident))),
            _ => Err(anyhow!("Unexpected token {:?}", next))
            
        }
    }
    fn parse(&mut self) -> Result<Expression> {
        let ast = self.primary()?;
        Ok(ast)
    }
}
impl TryFrom<Token> for Operator {
    type Error = &'static str;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[test]
fn test_iterator(){
    let tokenizer = Tokenizer::new("2+3+5/(2+1)".into());
    let mut i = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut i);

    let p = parser.parse().unwrap();
    println!("{:?}", p);
    // for token in tokenizer.iter() {
    //     println!("{:?}", token);
    // }
    //let parser = Parser::new(&mut tokenizer.iter().peekable());
    
}