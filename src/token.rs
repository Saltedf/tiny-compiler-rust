use crate::ast::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Single character
    LeftParen, // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    Minus,      // -
    Plus,       // +
    Semicolon,  // ;
    Slash,      // /
    Star,       // *
    Question,   // ?
    Colon,      // :
    NewLine,    // \n

    /// One or Two character
    Bang, // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >
    Less,         // <
    LessEqual,    // <=

    /// literals
    Name, //(String),
    Str,     // (String),
    Integer, //(i64),
    Float,   //(f64),

    /// Keywords
    And,
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Break,

    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    kind: Kind,
    lexeme: String,
    line: usize,
    pos: usize,
}

impl Range for Token {
    fn range(&self) -> (usize, usize) {
        (self.pos, self.pos + self.lexeme.len() - 1)
    }
}

impl TryFrom<Token> for i64 {
    type Error = std::num::ParseIntError;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        value.lexeme().parse::<i64>()
    }
}
impl TryFrom<Token> for f64 {
    type Error = std::num::ParseFloatError;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        value.lexeme().parse::<f64>()
    }
}

impl Token {
    pub fn new(kind: Kind, lexeme: String, line: usize, pos: usize) -> Self {
        Self {
            kind,
            lexeme,
            line,
            pos,
        }
    }
    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn len(&self) -> usize {
        self.lexeme.len()
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn range(&self) -> (usize, usize) {
        (self.pos, self.len())
    }
}
