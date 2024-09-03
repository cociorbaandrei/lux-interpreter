use crate::{
    parser::{Expression, Operator, Parser},
    tokenizer::Tokenizer,
};
use anyhow::{anyhow, Context};
use anyhow::{Ok, Result};
use std::ops::Add;
#[derive(Debug)]
enum RuntimeValue {
    Number(f64),
    Boolean(bool),
    String(String),
}
impl Add for RuntimeValue {
    type Output = Result<RuntimeValue>;

    fn add(self, other: RuntimeValue) -> Result<RuntimeValue> {
        match (self, other) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => Ok(RuntimeValue::Number(x + y)),
            (RuntimeValue::Number(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot add f64:{x} to bool:{y}")))
            }
            (RuntimeValue::Number(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot add f64:{x} to String:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Number(y)) => {
                Err(anyhow!(format!("Cannot add Boolean:{x} to Number:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot add Boolean:{x} to Boolean:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot add Boolean:{x} to String:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Number(y)) => {
                Err(anyhow!(format!("Cannot add String:{x} to Number:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot add String:{x} to Boolean:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::String(y)) => Ok(RuntimeValue::String(x + &y)),
        }
    }
}

impl Expression {
    pub fn eval(&self) -> Result<RuntimeValue> {
        match self {
            Expression::Binary(left, Operator::Add, right) => left.eval()? + right.eval()?,
            Expression::Binary(left, Operator::Multiply, right) => todo!(),
            Expression::Binary(left, Operator::Divide, right) => todo!(),
            Expression::Binary(left, Operator::Subtract, right) => todo!(),
            Expression::Binary(left, Operator::Power, right) => todo!(),
            Expression::Binary(left, Operator::EqualEqual, right) => todo!(),
            Expression::Binary(left, Operator::BangEqual, right) => todo!(),
            Expression::Binary(left, Operator::LessEqual, right) => todo!(),
            Expression::Binary(left, Operator::Less, right) => todo!(),
            Expression::Binary(left, Operator::Greater, right) => todo!(),
            Expression::Binary(left, Operator::GreaterEqual, right) => todo!(),
            Expression::Binary(left, Operator::Bang, right) => todo!(),
            Expression::Unary(Operator::Subtract, expr) => todo!(),
            Expression::Unary(Operator::Bang, expr) => todo!(),
            Expression::Number(val) => Ok(RuntimeValue::Number(*val)),
            Expression::Boolean(val) => Ok(RuntimeValue::Boolean(*val)),
            Expression::String(val) => Ok(RuntimeValue::String(val.into())),
            Expression::Group(expr) => expr.eval(),
            Expression::Nil => todo!(),
            _ => todo!(),
        }
    }
}

#[test]
fn test() {
    let tokenizer = Tokenizer::new("1+2+3+4+true".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);

    let p = parser.parse().unwrap().eval().unwrap();

    println!("{:?}", p);
}
