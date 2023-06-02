use std::{error::Error, fmt::format};

use crate::{
    reporter::ErrorReporter,
    token::{self, Kind, Token},
};

pub struct Scanner<'r> {
    reporter: &'r ErrorReporter,
    source: Vec<char>,
    /// 行号.
    line: usize,
    /// token的首个字符位置.
    start: usize,
    /// 对应peek() 所返回的字符, 即下个待分析的字符位置.
    current: usize,

    /// 生成的Token序列.
    tokens: Vec<Token>,
}

impl<'r> Scanner<'r> {
    pub fn new(source: &str, reporter: &'r ErrorReporter) -> Self {
        Self {
            reporter,
            source: source.chars().collect(),
            line: 1,
            start: 0,
            current: 0,
            tokens: vec![],
        }
    }
    pub fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.current += 1;
        ch
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new(
            Kind::Eof,
            "<EOF>".to_string(),
            self.line,
            self.current,
        ));

        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), Box<dyn Error>> {
        let next_char = self.advance().expect("Unexpected EOF.");

        match next_char {
            ' ' | '\t' | '\r' => (), // 跳过
            '\n' => {
                self.line += 1;
                self.add_token(Kind::NewLine);
            }
            '+' => self.add_token(Kind::Plus),
            '-' => {
                self.add_token(Kind::Minus);
            }
            '*' => self.add_token(Kind::Plus),
            '/' => {
                self.add_token(Kind::Minus);
            }
            ',' => self.add_token(Kind::Comma),
            '=' => {
                self.add_token(Kind::Equal);
            }
            '(' => self.add_token(Kind::LeftParen),
            ')' => self.add_token(Kind::RightParen),
            'A'..='Z' | 'a'..='z' | '_' => self.expect_ident()?,
            '0'..='9' => self.expect_number()?,
            o => self.reporter.error(
                &format!("Unexpected character: `{}`", o),
                self.line,
                self.current - 1,
                1,
            )?,
        }
        Ok(())
    }

    fn add_token(&mut self, kind: token::Kind) {
        let tk = Token::new(kind, self.current_lexeme(), self.line, self.start);
        self.tokens.push(tk)
    }

    #[inline]
    fn current_lexeme(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(
                self.source[self.start..self.current]
                    .iter()
                    .map(|c| *c as u8)
                    .collect(),
            )
        }
    }
    #[inline]
    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn expect_ident(&mut self) -> Result<(), Box<dyn Error>> {
        while !self.is_end() {
            match self.peek().unwrap() {
                '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' => {
                    self.advance();
                }
                _ => break,
            }
        }
        self.add_token(Kind::Name);
        Ok(())
    }

    fn expect_number(&mut self) -> Result<(), Box<dyn Error>> {
        let mut can_end = true;
        let mut kind = Kind::Integer;

        while !self.is_end() {
            match self.peek().unwrap() {
                '0'..='9' => {
                    self.advance();
                    can_end = true
                }
                '.' => {
                    if let Kind::Integer = kind {
                        self.advance();
                        kind = Kind::Float;
                        can_end = false;
                    } else {
                        self.reporter
                            .error("Unexpected `.`", self.line, self.current, 1)?
                    }
                }
                _ => {
                    if can_end {
                        break;
                    } else {
                        self.reporter.error(
                            "Expected a digit after `.`",
                            self.line,
                            self.current,
                            1,
                        )?
                    }
                }
            }
        }
        self.add_token(kind);
        Ok(())
    }
}
