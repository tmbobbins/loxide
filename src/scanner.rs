use crate::loxide::Loxide;
use crate::token::TokenType::{
    And, Bang, BangEqual, Class, Comma, Dot, Else, Eof, Equal, EqualEqual, False, For, Fun,
    Greater, GreaterEqual, Identifier, If, LeftBrace, LeftParen, Less, LessEqual, Minus, Nil,
    Number, Or, Plus, Print, Return, RightBrace, RightParen, Semicolon, Slash, Star, String_,
    Super, This, True, Var, While,
};
use crate::token::{Literal, Token, TokenType};
use phf::phf_map;
use std::collections::HashMap;
use std::str::FromStr;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => And,
    "class" => Class,
    "else" => Else,
    "false" => False,
    "for" => For,
    "fun" => Fun,
    "if" => If,
    "nil" => Nil,
    "or" => Or,
    "print" => Print,
    "return" => Return,
    "super" => Super,
    "this" => This,
    "true" => True,
    "var" => Var,
    "while" => While,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(Eof, "", None, self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let character = self.advance();
        match character {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => self.handle_multi_char_operators('=', BangEqual, Bang),
            '=' => self.handle_multi_char_operators('=', EqualEqual, Equal),
            '<' => self.handle_multi_char_operators('=', LessEqual, Less),
            '>' => self.handle_multi_char_operators('=', GreaterEqual, Greater),
            '/' => self.slash(),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => Loxide::error(self.line, "Unexpected character."),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap() //TODO: revisit get char iterator once, fix unwrap
    }

    fn handle_multi_char_operators(
        &mut self,
        second_character: char,
        true_: TokenType,
        false_: TokenType,
    ) {
        let type_ = self.token_match(second_character, true_, false_);
        self.add_token(type_);
    }

    fn token_match(&mut self, expected: char, true_: TokenType, false_: TokenType) -> TokenType {
        if self.match_token(expected) {
            return true_;
        }
        false_
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.get_char(self.current) != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn get_char(&self, index: usize) -> char {
        self.source.chars().nth(index).unwrap()
    }

    fn get_substring(&self, start: usize, end: usize) -> String {
        self.source[start..end].to_owned()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.get_char(self.current)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.get_char(self.current + 1)
    }

    fn slash(&mut self) {
        if self.match_token('/') {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
            return;
        }
        self.add_token(Slash);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Loxide::error(self.line, "Unterminated string.");
            return;
        }

        self.advance();
        self.add_token_literal(
            String_,
            Some(Literal::String_(
                self.get_substring(self.start + 1, self.current - 1),
            )),
        );
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token_literal(
            Number,
            Some(Literal::Number(
                f64::from_str(&self.source[(self.start + 1)..(self.current - 1)]).unwrap(),
            )),
        )
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        self.add_token(
            KEYWORDS
                .get(&self.get_substring(self.start, self.current))
                .map_or_else(|| Identifier, Clone::clone),
        );
    }

    const fn is_alphanumeric(&self, character: char) -> bool {
        character.is_ascii_alphanumeric() || character == '_'
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_token_literal(type_, None);
    }

    fn add_token_literal(&mut self, type_: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token::new(
            type_,
            &self.source[self.start..self.current],
            literal,
            self.line,
        ));
    }
}
