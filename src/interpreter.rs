use crate::expr::Expr;

pub struct Interpreter {
    expr: Expr
}

impl Interpreter {
    fn new(expr: Expr) -> Self { Self { expr} } 
}