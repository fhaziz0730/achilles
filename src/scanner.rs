use crate::token::Token;
use crate::token::TokenType;
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> 
    Self { Self { source, start: 0, current: 0, line: 1 } } 
    
    pub fn scan(&mut self) -> (Vec<Token>, Vec<String>){
    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<String> = vec![];
    while !(self.current >= self.source.len())
    {
        self.start = self.current;
        // scan token
        self.current = self.current + 1;
        let c = self.source.chars().nth(self.current-1);
        match c {
             Some(')') => {tokens.push(Token::new(TokenType::RightParen, String::new(), None,  self.line));}
             Some('{') => {tokens.push(Token::new(TokenType::LeftBrace, String::new(), None,  self.line));}
             Some('}') => {tokens.push(Token::new(TokenType::RightBrace, String::new(), None, self.line));}
             Some(',') => {tokens.push(Token::new(TokenType::Comma, String::new(), None, self.line));}
             Some('.') => {tokens.push(Token::new(TokenType::Dot, String::new(), None,  self.line));}
             Some('(') => {tokens.push(Token::new(TokenType::LeftParen, String::new(), None, self.line));}
             Some('-') => {tokens.push(Token::new(TokenType::Minus, String::new(), None, self.line));}
             Some('+') => {tokens.push(Token::new(TokenType::Plus, String::new(),None, self.line));}
             Some(';') => {tokens.push(Token::new(TokenType::Semicolon, String::new(), None, self.line));}
             Some('*') => {tokens.push(Token::new(TokenType::Star, String::new(), None, self.line));}
             Some('!') => {}
             None | Some(' ') | Some('\n') | Some('\t') | Some('\r') => {}
             Some(value) => {errors.push(format!("Character not recognized: {value}"));}
        } 
    }
    tokens.push(Token::new(TokenType::EOF, String::new(), None, self.line));
    (tokens, errors)
    }
}