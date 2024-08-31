use anyhow::Ok;
use anyhow::{Context, Result};
use anyhow::anyhow;
use std::fmt;
use std::path::Display;

pub enum Literal {
    None,
    Number(f64),
    String(String),
}
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::None => {
                write!(f, "NULL")
            },
            Literal::Number(x) => {
                write!(f, "{}", x)
            },
            Literal::String(s) => {
                write!(f, "{}", s)
            }
        }
    }
}