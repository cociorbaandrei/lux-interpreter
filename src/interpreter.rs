use crate::{
    parser::{Expression, Operator, Parser},
    tokenizer::Tokenizer,
};
use anyhow::{anyhow, Context};
use anyhow::{Ok, Result};
use std::ops::Add;
#[derive(Debug, PartialEq, PartialOrd)]
pub enum RuntimeValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil
}

impl RuntimeValue {
    pub fn multiply(self, other: RuntimeValue) -> Result<RuntimeValue> {
        match (self, other) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => Ok(RuntimeValue::Number(x * y)),
            (RuntimeValue::Number(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot multiply f64:{x} with bool:{y}")))
            }
            (RuntimeValue::Number(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot multiply f64:{x} with String:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Number(y)) => {
                Err(anyhow!(format!("Cannot multiply Boolean:{x} with Number:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot multiply Boolean:{x} with Boolean:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot multiply Boolean:{x} with String:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Number(y)) => {
                Err(anyhow!(format!("Cannot multiply String:{x} with Number:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot multiply String:{x} with Boolean:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::String(y)) => {
                Err(anyhow!("Cannot multiply two strings"))
            }
            (RuntimeValue::Nil, _) => Ok(RuntimeValue::Nil),
            (_, RuntimeValue::Nil) => Ok(RuntimeValue::Nil),
        }
    }
    pub fn divide(self, other: RuntimeValue) -> Result<RuntimeValue> {
        match (self, other) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => Ok(RuntimeValue::Number(x / y)),
            (RuntimeValue::Number(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot multiply f64:{x} with bool:{y}")))
            }
            (RuntimeValue::Number(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot multiply f64:{x} with String:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Number(y)) => {
                Err(anyhow!(format!("Cannot multiply Boolean:{x} with Number:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot multiply Boolean:{x} with Boolean:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot multiply Boolean:{x} with String:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Number(y)) => {
                Err(anyhow!(format!("Cannot multiply String:{x} with Number:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot multiply String:{x} with Boolean:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::String(y)) => {
                Err(anyhow!("Cannot multiply two strings"))
            }
            (RuntimeValue::Nil, _) => Ok(RuntimeValue::Nil),
            (_, RuntimeValue::Nil) => Ok(RuntimeValue::Nil),
        }
    }
    pub fn subtract(self, other: RuntimeValue) -> Result<RuntimeValue> {
        match (self, other) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => Ok(RuntimeValue::Number(x - y)),
            (RuntimeValue::Number(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot multiply f64:{x} with bool:{y}")))
            }
            (RuntimeValue::Number(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot multiply f64:{x} with String:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Number(y)) => {
                Err(anyhow!(format!("Cannot multiply Boolean:{x} with Number:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot multiply Boolean:{x} with Boolean:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot multiply Boolean:{x} with String:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Number(y)) => {
                Err(anyhow!(format!("Cannot multiply String:{x} with Number:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot multiply String:{x} with Boolean:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::String(y)) => {
                Err(anyhow!("Cannot multiply two strings"))
            }
            (RuntimeValue::Nil, _) => Ok(RuntimeValue::Nil),
            (_, RuntimeValue::Nil) => Ok(RuntimeValue::Nil),
        }
    }
    fn negate(&self) ->  Result<RuntimeValue> {
        match (self) {
            (RuntimeValue::Number(x)) => Ok(RuntimeValue::Number(-x)),
            (RuntimeValue::Boolean(x)) => {
               Err(anyhow!("Can not take the negative of a boolean."))
            }
            (RuntimeValue::String(x)) =>Err(anyhow!("Operand must be a number.")),
            RuntimeValue::Nil => Ok(RuntimeValue::Boolean(false)),
        }
    }

    pub fn not_and(self) -> Result<RuntimeValue> {
        match (self) {
            (RuntimeValue::Number(x)) => Ok(RuntimeValue::Boolean(!((x > 0.0)))),
            (RuntimeValue::Number(x)) => {
                Ok(RuntimeValue::Boolean(!((x > 0.0))))
            }
            (RuntimeValue::Boolean(x)) => {
                Ok(RuntimeValue::Boolean(!(x)))
            }
            (RuntimeValue::String(x)) => Ok(RuntimeValue::Boolean(false)),
            RuntimeValue::Number(x) => Ok(RuntimeValue::Boolean(x > 0.0)),
            RuntimeValue::Boolean(x) => Ok(RuntimeValue::Boolean(x)),
            RuntimeValue::String(x) => Ok(RuntimeValue::Boolean(false)),
            RuntimeValue::Nil => Ok(RuntimeValue::Boolean(true)),
        }
    }
    pub fn and(self, other: RuntimeValue) -> Result<RuntimeValue> {
        match (self, other) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => Ok(RuntimeValue::Boolean((x > 0.0) && (y > 0.0))),
            (RuntimeValue::Number(x), RuntimeValue::Boolean(y)) => {
                Ok(RuntimeValue::Boolean((x > 0.0) && y))
            }
            (RuntimeValue::Number(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot compare f64:{x} to String:{y}")))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Number(y)) => {
                Ok(RuntimeValue::Boolean((x && (y > 0.0))))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::Boolean(y)) => {
                Ok(RuntimeValue::Boolean(x && y))
            }
            (RuntimeValue::Boolean(x), RuntimeValue::String(y)) => {
                Err(anyhow!(format!("Cannot compare Boolean:{x} to String:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Number(y)) => {
                Err(anyhow!(format!("Cannot compare String:{x} to Number:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::Boolean(y)) => {
                Err(anyhow!(format!("Cannot compare String:{x} to Boolean:{y}")))
            }
            (RuntimeValue::String(x), RuntimeValue::String(y)) => Ok(RuntimeValue::String(x + &y)),
            (RuntimeValue::Number(_), RuntimeValue::Nil) =>Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Boolean(_), RuntimeValue::Nil) =>Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::String(_), RuntimeValue::Nil) => Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Nil, RuntimeValue::Number(_)) => Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Nil, RuntimeValue::Boolean(_)) => Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Nil, RuntimeValue::String(_)) => Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Nil, RuntimeValue::Nil) => Ok(RuntimeValue::Boolean(false)),
        }
    }
    pub fn less_than(self, other: RuntimeValue) -> Result<RuntimeValue> {
        match (self, other) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => Ok(RuntimeValue::Boolean(x<y)),
            _ => {
                Err(anyhow!(format!("Operands must be numbers.")))
            }
        }
    }
    pub fn less_than_equal(self, other: RuntimeValue) -> Result<RuntimeValue> {
        match (self, other) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => Ok(RuntimeValue::Boolean(x<=y)),
            _ => {
                Err(anyhow!(format!("Operands must be numbers.")))
            }
        }
    }
    pub fn greater(self, other: RuntimeValue) -> Result<RuntimeValue> {
        match (self, other) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => Ok(RuntimeValue::Boolean(x>y)),
            _ => {
                Err(anyhow!(format!("Operands must be numbers.")))
            }
        }
    }
    pub fn greater_equal(self, other: RuntimeValue) -> Result<RuntimeValue> {
        match (self, other) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => Ok(RuntimeValue::Boolean(x>=y)),
            _ => {
                Err(anyhow!(format!("Operands must be numbers.")))
            }
        }
    }
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
            (RuntimeValue::Number(_), RuntimeValue::Nil) => Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Boolean(_), RuntimeValue::Nil) => Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::String(_), RuntimeValue::Nil) =>Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Nil, RuntimeValue::Number(_)) =>Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Nil, RuntimeValue::Boolean(_)) => Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Nil, RuntimeValue::String(_)) => Ok(RuntimeValue::Boolean(false)),
            (RuntimeValue::Nil, RuntimeValue::Nil) => Ok(RuntimeValue::Boolean(false)),
        }
    }
}

impl Expression {
    pub fn eval(&self) -> Result<RuntimeValue> {
        match self {
            Expression::Binary(left, Operator::Add, right) => left.eval()? + right.eval()?,
            Expression::Binary(left, Operator::EqualEqual, right) => Ok(RuntimeValue::Boolean(left.eval()? == right.eval()?)),
            Expression::Binary(left, Operator::Multiply, right) => left.eval()?.multiply(right.eval()?),
            Expression::Binary(left, Operator::Divide, right) => left.eval()?.divide(right.eval()?),
            Expression::Binary(left, Operator::Subtract, right) => left.eval()?.subtract(right.eval()?),
            Expression::Binary(left, Operator::Power, right) => todo!(),
            Expression::Binary(left, Operator::BangEqual, right) => Ok(RuntimeValue::Boolean(left.eval()? != right.eval()?)),
            Expression::Binary(left, Operator::LessEqual, right) => left.eval()?.less_than_equal(right.eval()?),
            Expression::Binary(left, Operator::Less, right) => left.eval()?.less_than(right.eval()?),
            Expression::Binary(left, Operator::Greater, right) =>left.eval()?.greater(right.eval()?),
            Expression::Binary(left, Operator::GreaterEqual, right) => left.eval()?.greater_equal(right.eval()?),
            Expression::Binary(left, Operator::And, right) => left.eval()?.and(right.eval()?),
            Expression::Binary(left, Operator::Or, right) => todo!(),
            Expression::Unary(Operator::Subtract, expr) => expr.eval()?.negate(),
            Expression::Unary(Operator::Bang, expr) => expr.eval()?.not_and(),
            Expression::Number(val) => Ok(RuntimeValue::Number(*val)),
            Expression::Boolean(val) => Ok(RuntimeValue::Boolean(*val)),
            Expression::String(val) => Ok(RuntimeValue::String(val.into())),
            Expression::Group(expr) => expr.eval(),
            Expression::Nil => Ok(RuntimeValue::Nil),
            _ => todo!(),
        }
    }
}

#[test]
fn test_add() {
    let tokenizer = Tokenizer::new("1+2+3+4".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);

    let p = parser.parse().unwrap().eval().unwrap();

    println!("{:?}", p);
    assert_eq!(p, RuntimeValue::Number(10.0));
}
#[test]
fn test_add_strings() {
    let tokenizer = Tokenizer::new("\"Hello \" + \"World\"".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);

    let p = parser.parse().unwrap().eval().unwrap();

    println!("{:?}", p);
    assert_eq!(p, RuntimeValue::String("Hello World".into()));
}
#[test]
fn test_bool_logic() {
    let tokenizer = Tokenizer::new("4 < 7 and 7 < 9 and 2 < 5".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);
    //println!("{:?}", parser.parse().unwrap());
    let p = parser.parse().unwrap().eval().unwrap();

    println!("{:?}", p);
    assert_eq!(p, RuntimeValue::Boolean(true));
}
#[test]
fn test_bool_nil() {
    let tokenizer = Tokenizer::new("nil".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);
    //println!("{:?}", parser.parse().unwrap());
    let p = parser.parse().unwrap().eval().unwrap();

    println!("{:?}", p);
    assert_eq!(p, RuntimeValue::Nil);
}

#[test]
fn test_bool_nil_asd() {
    let tokenizer = Tokenizer::new("nil".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);
    //println!("{:?}", parser.parse().unwrap());
    let p = parser.parse().unwrap().eval().unwrap();

    println!("{:?}", p);
    assert_eq!(p, RuntimeValue::Nil);
}

#[test]
fn test_long_math_expr() {
    let tokenizer = Tokenizer::new("((2+5)/3 * (1+2+7)/2) / 0.25 * (1/2 + 2/3 + 4/5) + ((3/4 + 4/5) * 10) / 2 + 0.0723 + 0.60002222222222".into());
    let mut iter = tokenizer.iter().peekable();
    let mut parser = Parser::new(&mut iter);
   // println!("{:?}", parser.parse().unwrap());
    let p = parser.parse().unwrap().eval().unwrap();

    println!("{:?}", p);
    assert_eq!(p, RuntimeValue::Number(100.2001));
}