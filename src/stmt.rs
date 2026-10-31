use crate::{expr::Expr, token::Token};

pub enum Stmt{
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>)
}

impl std::fmt::Display for Stmt{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expression(expr) => {
                write!(f, "Expression {}", expr)
            },
            Stmt::Print(expr) => {
                write!(f, "Print expression {}", expr)
            },
            Stmt::Var(name, Some(expr)) => {
                write!(f, "Var {} {}", name, expr)
            },
            Stmt::Var(name, None) => {
                write!(f, "Var {}", name)
            }
        }
    }
}
