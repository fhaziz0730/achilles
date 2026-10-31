use crate::token::{Literal, Token};

pub enum Expr{
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Literal),
    Grouping(Box<Expr>),
    Variable(Token)
}

impl std::fmt::Display for Expr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(left, operator, right) => {
                write!(f, "({}{}{})", left, operator.lexeme(), right)
            },
            Expr::Unary(operator, expr) => {
                write!(f, "({}{})", operator.lexeme(), expr)
            },
            Expr::Literal(literal) => {
                write!(f, "{}", literal)
            },
            Expr::Grouping(expr) => {
                write!(f, "({})", expr)
            },
            Expr::Variable(name) => {
                write!(f, "var {}", name)
            }
        }
    }
}
