use crate::error_reporter;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;
use crate::error_reporter::ErrorReporter;
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: u32,
    tokens: Vec<Token>,
    errors: Vec<String>
}

impl Scanner {
    pub fn new(source: String) -> 
    Self { Self { source, start: 0, current: 0, line: 1, tokens: vec![], errors: vec![]} } 
    
    pub fn scan(&mut self, error_printer: ErrorReporter) -> (Vec<Token>, Vec<String>){
    while !self.is_at_end()
    {
        self.start = self.current;
        // scan token
        self.scan_token(&error_printer);
    }
    self.tokens.push(Token::new(TokenType::EOF, String::new(), None, self.line));
    (self.tokens.clone(), self.errors.clone())
    }

    fn scan_token(&mut self, error_printer: &ErrorReporter){
        
        let c = self.advance();
        match c {
             Some(')') => self.add_token(TokenType::RightParen),
             Some('{') => self.add_token(TokenType::LeftBrace),
             Some('}') => self.add_token(TokenType::RightBrace),
             Some(',') => self.add_token(TokenType::Comma),
             Some('.') => self.add_token(TokenType::Dot),
             Some('(') => self.add_token(TokenType::LeftParen),
             Some('-') => self.add_token(TokenType::Minus),
             Some('+') => self.add_token(TokenType::Plus),
             Some(';') => self.add_token(TokenType::Semicolon),
             Some('*') => self.add_token(TokenType::Star),
             Some('!') => if self.match_char('=') {self.add_token(TokenType::BangEqual)} else {self.add_token(TokenType::Bang)},
             Some('=') => if self.match_char('=') {self.add_token(TokenType::EqualEqual)} else {self.add_token(TokenType::Equal)},
             Some('<') => if self.match_char('<') {self.add_token(TokenType::LessEqual)} else {self.add_token(TokenType::Less)},
             Some('>') => if self.match_char('>') {self.add_token(TokenType::GreaterEqual)} else {self.add_token(TokenType::Greater)},
             None | Some(' ') | Some('\t') | Some('\r') => {},
             Some('\n') => {self.line = self.line + 1;},
             Some(value) => {error_printer.error(self.line, &format!("Character not recognized: {value}"));}
        } 
    }

    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char>{
        self.current = self.current + 1;
        let c = self.source.chars().nth(self.current-1);
        c
    }

    fn add_token(&mut self, token_type: TokenType){
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>){
        self.tokens.push(Token::new(token_type, String::new(), literal, self.line))
    }

    fn match_char(&mut self, c: char) -> bool{
        if self.is_at_end() {return false};
        if self.source.chars().nth(self.current) != Some(c){
            return false;
        }
        self.current = self.current + 1;
        return true;
    }
}