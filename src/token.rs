use core::fmt;
pub enum Literal{
    Number(f64),
    String(String),
    Boolean(bool),
    Nil
}

pub enum TokenType{
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Identifier, String, Number,
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    EOF
}


pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token_type {
            TokenType::LeftParen => write!(f, "( on Line {}", self.line),
            TokenType::RightParen => write!(f, ") on Line {}", self.line),
            TokenType::LeftBrace => write!(f, "{{ on Line {}", self.line),
            TokenType::RightBrace => write!(f, "}} on Line {}", self.line),
            TokenType::Comma => write!(f, ", on Line {}", self.line),
            TokenType::Dot=> write!(f, ". on Line {}", self.line),
            TokenType::Minus =>  write!(f, "- on Line {}", self.line),
            TokenType::Plus => write!(f, "+ on Line {}", self.line),
            TokenType::Semicolon => write!(f, "; on Line {}", self.line),
            TokenType::Slash => write!(f, "/ on Line {}", self.line),
            TokenType::Star => write!(f, "* on Line {}", self.line),
            TokenType::Bang => write!(f, "! on Line {}", self.line),
            TokenType::BangEqual => write!(f, "!= on Line {}", self.line),
            TokenType::Equal => write!(f, "= on Line {}", self.line),
            TokenType::EqualEqual => write!(f, "== on Line {}", self.line),
            TokenType::Greater => write!(f, "> on Line {}", self.line),
            TokenType::GreaterEqual => write!(f, ">= on Line {}", self.line),
            TokenType::Less => write!(f, "< on Line {}", self.line),
            TokenType::LessEqual => write!(f, "<= on Line {}", self.line),
            TokenType::Identifier => write!(f, "identifier {} on Line {}", self.lexeme, self.line),
            TokenType::String => write!(f, "string {} on Line {}", self.lexeme, self.line),
            TokenType::Number => write!(f, "number {} on Line {}", self.lexeme, self.line),
            TokenType::And => write!(f, "keyword AND on Line {}", self.line),
            TokenType::Class => write!(f, "keyword Class on Line {}", self.line),
            TokenType::Else => write!(f, "keyword Else on Line {}", self.line),
            TokenType::False => write!(f, "keyword False on Line {}", self.line),
            TokenType::Fun => write!(f, "keyword Fun on Line {}", self.line),
            TokenType::For => write!(f, "keyword For on Line {}", self.line),
            TokenType::If => write!(f, "keyword If on Line {}", self.line),
            TokenType::Nil => write!(f, "keyword Nil on Line {}", self.line),
            TokenType::Or => write!(f, "keyword Or on Line {}", self.line),
            TokenType::Print => write!(f, "keyword Print on Line {}", self.line),
            TokenType::Return => write!(f, "keyword Return on Line {}", self.line),
            TokenType::Super => write!(f, "keyword Super on Line {}", self.line),
            TokenType::This => write!(f, "keyword This on Line {}", self.line),
            TokenType::True => write!(f, "keyword True on Line {}", self.line),
            TokenType::Var => write!(f, "keyword Var on Line {}", self.line),
            TokenType::While => write!(f, "keyword While on Line {}", self.line),
            TokenType::EOF => write!(f, "EOF on Line {}", self.line)
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: u32) -> 
    Self { Self { token_type, lexeme, literal, line } } 
    pub fn token_type(&self) -> &TokenType { &self.token_type }
    pub fn lexeme(&self) -> &String { &self.lexeme }
    pub fn line(&self) -> u32 { self.line }
    pub fn literal(&self) -> Option<&Literal> {
        self.literal.as_ref()
    }
}