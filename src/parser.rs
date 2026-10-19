use crate::{expr::Expr::{self, Binary, Grouping, Literal, Unary}, token::{self, Literal::Boolean, Token, TokenType::{self, Bang, BangEqual, EOF, EqualEqual, False, Greater, GreaterEqual, LeftParen, Less, LessEqual, Minus, Nil, Number, Plus, RightParen, Slash, Star, True}}};

pub struct Parser {
    current: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> 
    Self { Self { current: 0, tokens} } 
    fn expression(&mut self) -> Expr{
        return self.equality();
    }
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token(vec![BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }
    fn match_token(&mut self, types: Vec<TokenType>) -> bool{
        for token_type in &types{
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type() == token_type;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type() == &EOF
    }
    fn peek(&self) -> &Token {
        return self.tokens.get(self.current).expect("current should not be invalid");
    }
    fn previous(&self) -> &Token {
        return self.tokens.get(self.current - 1).expect("current should not be invalid");
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_token(vec![Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(vec![Slash, Star]){
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }
    fn unary(&mut self) -> Expr {
        if self.match_token(vec![Bang, Minus]) {
            let operator  = self.previous().clone();
            let right = self.unary();
            return Unary(operator, Box::new(right));
        }
        return self.primary();
    }
    fn primary(&mut self) -> Expr {
        if self.match_token(vec![False]) {return Literal(Boolean(false))}
        if self.match_token(vec![True]) {return Literal(Boolean(true))}
        if self.match_token(vec![Nil]) {return Literal(token::Literal::Nil)}
        if self.match_token(vec![Number, token::TokenType::String]) {return Literal(self.previous().literal().cloned().expect("Should have literal"))}
        if self.match_token(vec![LeftParen]) {
            let expr = self.expression();
            self.consume(&RightParen, String::from("Exprect ')' after expression"));
            return Grouping(Box::new(expr));
        }
        panic!("No way!")
    }
    fn consume(&mut self, token_type: &TokenType, message: String) -> Token {
        if self.check(token_type) { return self.advance().clone();}
        panic!("{}{}", self.peek(), message);
    }
    pub fn parse (&mut self) -> Expr {
        return self.expression();
    }
}