use crate::error_reporter;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;
use crate::error_reporter::ErrorReporter;
pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
    tokens: Vec<Token>,
    errors: Vec<String>
}

impl Scanner {
    pub fn new(source: String) -> 
    Self { Self { source: source.chars().collect(), start: 0, current: 0, line: 1, tokens: vec![], errors: vec![]} } 
    
    pub fn scan(mut self, error_printer: ErrorReporter) -> (Vec<Token>, Vec<String>){
    while !self.is_at_end()
    {
        self.start = self.current;
        // scan token
        self.scan_token(&error_printer);
    }
    self.tokens.push(Token::new(TokenType::EOF, String::new(), None, self.line));
    (self.tokens, self.errors)
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
    
    fn advance(&mut self) -> Option<char> {
        let c = self.source.get(self.current).copied();
        if c.is_some(){
            self.current += 1;
        }
        c
    }
    
    fn match_char(&mut self, expected: char) -> bool {
        if self.source.get(self.current) == Some(&expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, token_type: TokenType){
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(
        &mut self,
        token_type: TokenType,
        literal: Option<Literal>
    ) {
        let lexeme: String = self.source[self.start..self.current]
            .iter()
            .collect();

        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
        }
}