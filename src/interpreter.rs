use crate::{expr::Expr::{self}, stmt::Stmt, token::{Literal, Token, TokenType}};


pub struct Interpreter {
    
}

impl Interpreter {
    pub fn new() -> Self { Self { } } 
    pub fn interpret(&self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts{
            self.execute(stmt)?;
        }
        Ok(())
    }
    fn execute(&self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {self.evaluate(expr)?; Ok(())},
            Stmt::Print(value) => {
                let v = self.evaluate(value)?;
                println!("{}", v);
                Ok(())
            }
        }
    }
    fn evaluate (&self, expr: Expr) -> Result<Literal, String> {
         return match expr {
            Expr::Literal(literal) => Ok(literal),
            Expr::Grouping(expr_tail) => self.evaluate(*expr_tail),
            Expr::Unary(op, right) => self.evaluate_unary(op, self.evaluate(*right)?),
            Expr::Binary(left, op, right) => self.evaluate_binary(op, self.evaluate(*left)?, self.evaluate(*right)?)
        }
    }
    fn evaluate_unary(&self, op: Token, expr : Literal) -> Result<Literal, String> {
        return match (op.token_type(), expr) {
            (TokenType::Minus, Literal::Number(x)) => Ok(Literal::Number(-x)),
            (TokenType::Bang, x) => Ok(Literal::Boolean(!self.is_truthy(x))),
            (TokenType::Minus, _) => Err(format!("{} Operand must be a number", op)),
            _ => Err(format!("Can't do that operation"))
        }
    }
    fn evaluate_binary(&self, op:Token, left: Literal, right: Literal) -> Result<Literal, String> {
        return match (op.token_type(), left, right) {
            (TokenType::Minus, Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x - y)),
            (TokenType::Minus, _, _) => Err(format!(" {} Operands must be numbers", op)),
            (TokenType::Slash, Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x/y)),
            (TokenType::Slash, _, _) => Err(format!(" {} Operands must be numbers", op)),
            (TokenType::Star, Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x * y)),
            (TokenType::Star, _, _) => Err(format!(" {} Operands must be numbers", op)),
            (TokenType::Plus, Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x + y)),
            (TokenType::Plus, Literal::String(x), Literal::String(y)) => Ok(Literal::String(format!("{}{}", x, y))),
            (TokenType::Plus, _, _) => Err(format!(" {} Operands must be numbers or strings", op)),
            (TokenType::Greater, Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean( x > y)),
            (TokenType::Greater, _, _) => Err(format!("{} Operands must be numbers", op)),
            (TokenType::GreaterEqual, Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean( x >= y)),
            (TokenType::GreaterEqual, _, _) => Err(format!("{} Operands must be numbers", op)),
            (TokenType::Less, Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean( x < y)),
            (TokenType::Less, _, _) => Err(format!("{} Operands must be numbers", op)),
            (TokenType::LessEqual, Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean( x <= y)),
            (TokenType::LessEqual, _, _) => Err(format!("{} Operands must be numbers", op)),
            (TokenType::BangEqual, x, y) => Ok(Literal::Boolean(!self.is_equal(x, y))),
            (TokenType::EqualEqual, x, y) => Ok(Literal::Boolean(self.is_equal(x, y))),
            _ => Err(format!("Operator is not supported for binary expression"))
        }
    }
    fn is_truthy(&self, literal: Literal) -> bool {
        return match literal {
            Literal::Nil => false,
            Literal::Boolean(bool) => bool,
            _ => true
        }
    }
    fn is_equal(&self, left: Literal, right: Literal) -> bool {
        return match (left, right) {
            (Literal::Number(x), Literal::Number(y)) => x == y,
            (Literal::Nil, Literal::Nil) => true,
            (Literal::Boolean(x), Literal::Boolean(y)) => x ==y,
            (Literal::String(x), Literal::String(y)) => x == y,
            _ => false
        }
    }
}