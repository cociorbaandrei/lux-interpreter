use std::fmt::Binary;
use std::iter::Peekable;
use std::os::linux::raw::stat;

use crate::parser;
use crate::tokenizer::Tokenizer;
use crate::{token::Token, tokenizer::TokenIter};
use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use anyhow::{anyhow, Context};
#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
    Power,
    EqualEqual,
    BangEqual,
    LessEqual,
    Less,
    Greater,
    GreaterEqual,
    Bang,
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary(Box<Expression>, Operator, Box<Expression>),
    Unary(Operator, Box<Expression>),
    Number(f64),
    Boolean(bool),
    String(String),
    Group(Box<Expression>),
    Nil,
}

#[derive(Debug)]
pub enum Statement {
    Print(Box<Expression>),
    ExprStmt(Box<Expression>),
    Program(Vec<Statement>),
}

impl Expression {
    pub fn pprint(&self) -> String {
        match self {
            Expression::Binary(left, op, right) => {
                let op = match op {
                    Operator::Add => "+".to_string(),
                    Operator::Multiply => "*".to_string(),
                    Operator::Divide => "/".to_string(),
                    Operator::Subtract => "-".to_string(),
                    Operator::Power => todo!(),
                    Operator::EqualEqual => "==".to_string(),
                    Operator::BangEqual => "!=".to_string(),
                    Operator::LessEqual => "<=".to_string(),
                    Operator::Less => "<".to_string(),
                    Operator::Greater => ">".to_string(),
                    Operator::GreaterEqual => ">=".to_string(),
                    Operator::Bang => "!".to_string(),
                    Operator::And => "&&".to_string(),
                    Operator::Or => "||".to_string(),
                };
                return "(".to_owned() + &op + " " + &left.pprint() + " " + &right.pprint() + ")";
            }
            Expression::Unary(op, expr) => {
                let op = match op {
                    Operator::Add => "+".to_string(),
                    Operator::Multiply => "*".to_string(),
                    Operator::Divide => "/".to_string(),
                    Operator::Subtract => "-".to_string(),
                    Operator::Power => todo!(),
                    Operator::EqualEqual => "==".to_string(),
                    Operator::BangEqual => "!=".to_string(),
                    Operator::LessEqual => "<=".to_string(),
                    Operator::Less => "<".to_string(),
                    Operator::Greater => ">".to_string(),
                    Operator::GreaterEqual => ">=".to_string(),
                    Operator::Bang => "!".to_string(),
                    Operator::And => "&&".to_string(),
                    Operator::Or => "||".to_string(),
                };
                return "(".to_owned() + &op + " " + &expr.pprint() + ")";
            }
            Expression::Number(val) => {
                if val.trunc() == *val {
                    format!("{:.1}", val)
                } else {
                    format!("{}", val)
                }
            }
            Expression::Boolean(val) => val.to_string(),
            Expression::String(val) => val.to_owned(),
            Expression::Nil => "nil".to_owned(),
            Expression::Group(expr) => "(group ".to_owned() + &expr.pprint() + ")",
        }
    }
}
pub struct Parser<'a> {
    iter: &'a mut Peekable<TokenIter<'a>>,
}
impl<'a> Parser<'a> {
    pub fn new(iter: &'a mut Peekable<TokenIter<'a>>) -> Self {
        Parser { iter: iter }
    }
    fn assert_next(&mut self, token: Token) -> Result<()> {
        let next = self.iter.next();
        if next == None {
            return Err(anyhow!("Undexpected end of input."));
        }
        if next.as_ref() != Some(&token) {
            return Err(anyhow!("Expected {:?} actual {:?}", token, next.as_ref()));
        }
        Ok(())
    }
    fn operator(&mut self) -> Result<Operator> {
        let next = self.next().context("Expected operator, got EOF")?;
        let op = match next {
            Token::LeftParen(_, _, _) => todo!(),
            Token::RightParen(_, _, _) => todo!(),
            Token::LeftBrace(_, _, _) => todo!(),
            Token::RightBrace(_, _, _) => todo!(),
            Token::Star(_, _, _) => Operator::Multiply,
            Token::Dot(_, _, _) => todo!(),
            Token::Comma(_, _, _) => todo!(),
            Token::Plus(_, _, _) => Operator::Add,
            Token::Minus(_, _, _) => Operator::Subtract,
            Token::Semicolon(_, _, _) => todo!(),
            Token::Slash(_, _, _) => Operator::Divide,
            Token::Equal(_, _, _) => todo!(),
            Token::EqualEqual(_, _, _) => todo!(),
            Token::Bang(_, _, _) => todo!(),
            Token::BangEqual(_, _, _) => todo!(),
            Token::Less(_, _, _) => todo!(),
            Token::LessEqual(_, _, _) => todo!(),
            Token::Greater(_, _, _) => todo!(),
            Token::GreaterEqual(_, _, _) => todo!(),
            Token::Colon(_, _, _) => todo!(),
            Token::Error(_) => todo!(),
            Token::Number(_, _, _, _) => todo!(),
            Token::Identifier(_, _, _, _) => todo!(),
            Token::String(_, _, _, _) => todo!(),
            Token::And(_, _, _) => todo!(),
            Token::Class(_, _, _) => todo!(),
            Token::Else(_, _, _) => todo!(),
            Token::False(_, _, _) => todo!(),
            Token::For(_, _, _) => todo!(),
            Token::Fun(_, _, _) => todo!(),
            Token::If(_, _, _) => todo!(),
            Token::Nil(_, _, _) => todo!(),
            Token::Or(_, _, _) => todo!(),
            Token::Print(_, _, _) => todo!(),
            Token::Return(_, _, _) => todo!(),
            Token::Super(_, _, _) => todo!(),
            Token::This(_, _, _) => todo!(),
            Token::True(_, _, _) => todo!(),
            Token::Var(_, _, _) => todo!(),
            Token::While(_, _, _) => todo!(),
            Token::EndOfFile => todo!(),
        };
        Ok(op)
    }
    fn primary(&mut self) -> Result<Expression> {
        let next = self.iter.next().context("Expected token got EOF")?;
        match next {
            Token::LeftParen(_, line, col) => {
                let e = self.expression()?;
                let expected = self
                    .iter
                    .next()
                    .context(format!("Expected ')' at line:{line} col:{col}"))?;

                match expected {
                    Token::RightParen(_, _, _) => Ok(Expression::Group(Box::new(e))),
                    _ => Err(anyhow!("Expected ')' at line:{line} col:{col}")),
                }
            }
            Token::Number(_, _, _, n) => Ok(Expression::Number(n)),
            Token::True(_, _, _) => Ok(Expression::Boolean(true)),
            Token::False(_, _, _) => Ok(Expression::Boolean(false)),
            Token::Nil(_, _, _) => Ok(Expression::Nil),
            Token::String(_, _, _, s) => Ok(Expression::String(s)),
            _ => Err(anyhow!("Unexpected")),
        }
    }
    fn uanary(&mut self) -> Result<Expression> {
        let next = self.iter.peek().context("Unexpected EOF.")?;
        match next {
            Token::Bang(_, _, _) => {
                self.iter.next();
                Ok(Expression::Unary(Operator::Bang, Box::new(self.uanary()?)))
            }
            Token::Minus(_, _, _) => {
                self.iter.next();

                Ok(Expression::Unary(
                    Operator::Subtract,
                    Box::new(self.uanary()?),
                ))
            }
            _ => self.primary(),
        }
    }
    fn factor(&mut self) -> Result<Expression> {
        let mut left = self.uanary()?;
        loop {
            let next = self.iter.peek();
            match next {
                Some(Token::Star(_, _, _)) => {
                    self.iter.next();
                    let right = self.uanary()?;
                    left = Expression::Binary(Box::new(left), Operator::Multiply, Box::new(right));
                }
                Some(Token::Slash(_, _, _)) => {
                    self.iter.next();
                    let right = self.uanary()?;
                    left = Expression::Binary(Box::new(left), Operator::Divide, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    fn term(&mut self) -> Result<Expression> {
        let mut left = self.factor()?;
        loop {
            let next = self.iter.peek();
            match next {
                Some(Token::Plus(_, _, _)) => {
                    self.iter.next();
                    let right = self.factor()?;
                    left = Expression::Binary(Box::new(left), Operator::Add, Box::new(right));
                }
                Some(Token::Minus(_, _, _)) => {
                    self.iter.next();
                    let right = self.factor()?;
                    left = Expression::Binary(Box::new(left), Operator::Subtract, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    fn comparison(&mut self) -> Result<Expression> {
        let mut left = self.term()?;
        loop {
            let op: Option<&Token> = self.iter.peek();
            match op {
                Some(Token::Greater(_, _, _)) => {
                    self.iter.next();
                    let right = self.term()?;
                    left = Expression::Binary(Box::new(left), Operator::Greater, Box::new(right));
                }
                Some(Token::GreaterEqual(_, _, _)) => {
                    self.iter.next();
                    let right = self.term()?;
                    left =
                        Expression::Binary(Box::new(left), Operator::GreaterEqual, Box::new(right));
                }
                Some(Token::Less(_, _, _)) => {
                    self.iter.next();
                    let right = self.term()?;
                    left = Expression::Binary(Box::new(left), Operator::Less, Box::new(right));
                }
                Some(Token::LessEqual(_, _, _)) => {
                    self.iter.next();
                    let right = self.term()?;
                    left = Expression::Binary(Box::new(left), Operator::LessEqual, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut left = self.comparison()?;
        loop {
            let op = self.iter.peek();
            match op {
                Some(Token::BangEqual(_, _, _)) => {
                    self.iter.next();
                    let right = self.comparison()?;
                    left = Expression::Binary(Box::new(left), Operator::BangEqual, Box::new(right));
                }
                Some(Token::EqualEqual(_, _, _)) => {
                    self.iter.next();
                    let right = self.comparison()?;
                    left =
                        Expression::Binary(Box::new(left), Operator::EqualEqual, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    fn and(&mut self) -> Result<Expression> {
        let mut left = self.equality()?;
        loop {
            let op = self.iter.peek();
            match op {
                Some(Token::And(_, _, _)) => {
                    self.iter.next();
                    let right = self.equality()?;
                    left = Expression::Binary(Box::new(left), Operator::And, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    fn or(&mut self) -> Result<Expression> {
        let mut left = self.and()?;
        loop {
            let op = self.iter.peek();
            match op {
                Some(Token::Or(_, _, _)) => {
                    self.iter.next();
                    let right = self.and()?;
                    left = Expression::Binary(Box::new(left), Operator::Or, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    fn expression(&mut self) -> Result<Expression> {
        let mut expr = self.or()?;
        Ok(expr)
    }
    fn print_stmt(&mut self) -> Result<Statement> {
        let print = self.iter.next().context("Expected print keyword.")?;
        let print_stmt = self.expression()?;
        let semi = self.iter.next().context("Expected ; after expression.")?;
        Ok(Statement::Print(Box::new(print_stmt)))
    }
    fn expr_stmt(&mut self) -> Result<Statement> {
        let expr_stmt = self.expression()?;
        let semi = self.iter.next().context("Expected ; after expression.")?;
        Ok(Statement::ExprStmt(Box::new(expr_stmt)))
    }
    fn statement(&mut self) -> Result<Statement> {
        let next = self.iter.peek();
        match next {
            Some(Token::Print(_, _, _)) => self.print_stmt(),
            _ => self.expr_stmt(),
        }
    }
    fn program(&mut self) -> Result<Statement> {
        let mut statements = Vec::new();
        while let Some(token) = self.iter.peek() {
            statements.push(self.statement()?);
        }
        Ok(Statement::Program(statements))
    }
    pub fn parse_program(&mut self) -> Result<Statement> {
        let ast = self.program()?;
        Ok(ast)
    }
    pub fn parse(&mut self) -> Result<Expression> {
        let ast = self.expression()?;
        Ok(ast)
    }
    fn peek(&mut self) -> Option<&Token> {
        self.iter.peek()
    }
    fn next(&mut self) -> Option<Token> {
        self.iter.next()
    }
}
impl TryFrom<Token> for Operator {
    type Error = &'static str;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[test]
fn test_iterator() {
    let tokenizer = Tokenizer::new("1 / 2 * 3 / 5   ".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);

    let p = parser.parse().unwrap();
    assert_eq!(
        p,
        Expression::Binary(
            Box::new(Expression::Binary(
                Box::new(Expression::Binary(
                    Box::new(Expression::Number(1.0)),
                    Operator::Divide,
                    Box::new(Expression::Number(2.0))
                )),
                Operator::Multiply,
                Box::new(Expression::Number(3.0))
            )),
            Operator::Divide,
            Box::new(Expression::Number(5.0))
        )
    );
    println!("{:?}", p);
    // for token in tokenizer.iter() {
    //     println!("{:?}", token);
    // }
    //let parser = Parser::new(&mut tokenizer.iter().peekable());
}

#[test]
fn test_term() {
    let tokenizer = Tokenizer::new("1 + 2 / 3".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);

    let p = parser.parse().unwrap();

    println!("{:?}", p);
    assert_eq!(
        p,
        Expression::Binary(
            Box::new(Expression::Number(1.0)),
            Operator::Add,
            Box::new(Expression::Binary(
                Box::new(Expression::Number(2.0)),
                Operator::Divide,
                Box::new(Expression::Number(3.0))
            ))
        )
    )
    // for token in tokenizer.iter() {
    //     println!("{:?}", token);
    // }
    //let parser = Parser::new(&mut tokenizer.iter().peekable());
}

#[test]
fn test_unary() {
    let tokenizer = Tokenizer::new("-1 + 2 / -3".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);

    let p = parser.parse().unwrap();

    println!("{:?}", p);
    assert_eq!(
        p,
        Expression::Binary(
            Box::new(Expression::Unary(
                Operator::Subtract,
                Box::new(Expression::Number(1.0))
            )),
            Operator::Add,
            Box::new(Expression::Binary(
                Box::new(Expression::Number(2.0)),
                Operator::Divide,
                Box::new(Expression::Unary(
                    Operator::Subtract,
                    Box::new(Expression::Number(3.0))
                ))
            ))
        )
    )
    // for token in tokenizer.iter() {
    //     println!("{:?}", token);
    // }
    //let parser = Parser::new(&mut tokenizer.iter().peekable());
}

#[test]
fn test_boolean() {
    use Expression::*;
    use Operator::*;

    let tokenizer = Tokenizer::new("(4+2)*3 == 18".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);

    let p = parser.parse().unwrap();

    println!("{:?}", p);
    println!("{}", p.pprint());
    // assert_eq!(
    //     p,
    //     Binary(
    //         Box::new(Binary(
    //             Box::new(Binary(Box::new(Number(4.0)), Add, Box::new(Number(2.0)))),
    //             Multiply,
    //             Box::new(Number(3.0))
    //         )),
    //         EqualEqual,
    //         Box::new(Number(18.0))
    //     )
    // );
    // for token in tokenizer.iter() {
    //     println!("{:?}", token);
    // }
    //let parser = Parser::new(&mut tokenizer.iter().peekable());
}

#[test]
fn test_boolean_2() {
    use Expression::*;
    use Operator::*;

    let tokenizer = Tokenizer::new("!true and true".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);

    let p = parser.parse().unwrap();

    println!("{:?}", p);
    println!("{}", p.pprint());
}
