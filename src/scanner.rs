use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;
use crate::error_reporter::ErrorReporter;
pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source: String) -> 
    Self { Self { source: source.chars().collect(), start: 0, current: 0, line: 1, tokens: vec![]} } 
    
    pub fn scan(mut self, error_printer: &mut ErrorReporter) -> Vec<Token>{
    while !self.is_at_end()
    {
        self.start = self.current;
        // scan token
        self.scan_token(error_printer);
    }
    self.tokens.push(Token::new(TokenType::EOF, String::new(), None, self.line));
    self.tokens
    }

    fn scan_token(&mut self, error_printer: &mut ErrorReporter){
        
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
             Some('<') => if self.match_char('=') {self.add_token(TokenType::LessEqual)} else {self.add_token(TokenType::Less)},
             Some('>') => if self.match_char('=') {self.add_token(TokenType::GreaterEqual)} else {self.add_token(TokenType::Greater)},
             Some('/') => {
                if self.match_char('/'){
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
             }
             Some('"') => self.string(error_printer),
             None | Some(' ') | Some('\t') | Some('\r') => {},
             Some('\n') => {self.line = self.line + 1;},
             Some(value) => {
                if value.is_ascii_digit() {self.number()}
                else if self.is_alpha(value) {self.identifier()}
                else {error_printer.error(self.line, &format!("Character not recognized: {value}"));}
             }
        } 
    }

    fn number(&mut self){
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }
        if self.peek() == Some('.')
            && self.peek_next().is_some_and(|c| c.is_ascii_digit())
        {
            self.advance();

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }
        let value: f64 = self.source[self.start..self.current]
        .iter()
        .collect::<String>()
        .parse()
        .expect("Invalid number");

        self.add_token_with_literal(
            TokenType::Number,
            Some(Literal::Number(value)),
        );
    }

    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }
    
    fn string(&mut self, error_printer: &mut ErrorReporter){
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') { self.line = self.line + 1;}
            self.advance();
        }
        if self.is_at_end() {
            error_printer.error(self.line, "Unterminated string");
            return;
        }
        self.advance();
        let value = Literal::String(
            self.source[self.start + 1..self.current - 1]
                .iter()
                .collect()
        );
        self.add_token_with_literal(TokenType::String, Some(value));
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.get(self.current).copied();
        if c.is_some(){
            self.current += 1;
        }
        c
    }
    
    fn peek(&self) -> Option<char>{
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char>{
        self.source.get(self.current + 1).copied()
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
    
    fn identifier(&mut self){
        while self
            .peek()
            .is_some_and(|c| self.is_alpha_numeric(c))
        {
            self.advance();
        }
        let text: String = self.source[self.start..self.current]
                        .iter()
                        .collect();
        let text = text.as_str();
        match text {
            "and" => self.add_token(TokenType::And),
            "class" => self.add_token(TokenType::Class),
            "else" => self.add_token(TokenType::Else),
            "false" => self.add_token(TokenType::False),
            "for" => self.add_token(TokenType::For),
            "fun" => self.add_token(TokenType::Fun),
            "if" => self.add_token(TokenType::If),
            "nil" => self.add_token(TokenType::Nil),
            "or" => self.add_token(TokenType::Or),
            "print" => self.add_token(TokenType::Print),
            "return" => self.add_token(TokenType::Return),
            "super" => self.add_token(TokenType::Super),
            "this" => self.add_token(TokenType::This),
            "true" => self.add_token(TokenType::True),
            "var" => self.add_token(TokenType::Var),
            "while" => self.add_token(TokenType::While),
            _ => self.add_token(TokenType::Identifier)
        }
    }

    fn is_alpha(&self, c: char) -> bool{
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool{
        self.is_alpha(c) || c.is_ascii_digit()
    }
}