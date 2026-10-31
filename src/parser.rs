use crate::{expr::Expr::{self, Binary, Grouping, Literal, Unary}, stmt::Stmt, token::{self, Literal::Boolean, Token, TokenType::{self, Bang, BangEqual, EOF, Equal, EqualEqual, False, Greater, GreaterEqual, Identifier, LeftParen, Less, LessEqual, Minus, Nil, Number, Plus, RightParen, Semicolon, Slash, Star, True}}};

pub struct Parser {
    current: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> 
    Self { Self { current: 0, tokens} } 
    fn expression(&mut self) -> Result<Expr, String>{
        return self.equality();
    }
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_token(vec![BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }
        return Ok(expr);
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
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_token(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }
        return Ok(expr);
    }
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_token(vec![Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }
        return Ok(expr);
    }
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_token(vec![Slash, Star]){
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }
        return Ok(expr);
    }
    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(vec![Bang, Minus]) {
            let operator  = self.previous().clone();
            let right = self.unary();
            return Ok(Unary(operator, Box::new(right?)));
        }
        return self.primary();
    }
    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(vec![False]) {return Ok(Literal(Boolean(false)))}
        if self.match_token(vec![True]) {return Ok(Literal(Boolean(true)))}
        if self.match_token(vec![Nil]) {return Ok(Literal(token::Literal::Nil))}
        if self.match_token(vec![Identifier]) {return Ok(Expr::Variable(self.previous().clone()))}
        if self.match_token(vec![Number, token::TokenType::String]) {return Ok(Literal(self.previous().literal().cloned().expect("Should have literal")))}
        if self.match_token(vec![LeftParen]) {
            let expr = self.expression()?;
            self.consume(&RightParen, String::from("Expect ')' after expression"))?;
            return Ok(Grouping(Box::new(expr)));
        }
        Err(format!("Expect expression {}", self.peek()))
    }
    fn consume(&mut self, token_type: &TokenType, message: String) -> Result<Token, String> {
        if self.check(token_type) { return Ok(self.advance().clone());}
        Err(format!("{} {}", self.peek(), message))
    }
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        return Ok(statements)
    }
    pub fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(vec![TokenType::Var]) {
            return self.var_declaration();
        }
        return self.statement();
    }
    pub fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = 
            self.consume(&Identifier, String::from("Expect var name"))?;
        let mut initializer: Option<Expr> = None;
        if self.match_token(vec![Equal]) {
            initializer = Some(self.expression()?);
        }
        self.consume(&Semicolon, String::from("Expect ';' after var declaration."))?;
        return Ok(Stmt::Var(name, initializer))
    }
    pub fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(vec![token::TokenType::Print]) {
            return self.print_statement();
        }
        return self.expression_statement();
    }
    fn print_statement(&mut self) -> Result<Stmt, String>{
        let value = self.expression()?;
        self.consume(&token::TokenType::Semicolon, String::from("Expect ; after value."))?;
        return Ok(Stmt::Print(value));
    }
    fn expression_statement(&mut self) -> Result<Stmt, String>{
        let expr = self.expression()?;
        self.consume(&token::TokenType::Semicolon, String::from("Expect ; after expression."))?;
        return Ok(Stmt::Expression(expr));
    }
}