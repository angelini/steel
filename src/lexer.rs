use std::{
    iter::Peekable,
    str::{Chars, Lines},
};

#[derive(Clone, Copy, Debug)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    fn new() -> Self {
        Location { line: 0, column: 0 }
    }

    fn back_one(&self) -> Self {
        Self {
            line: self.line,
            column: if self.column > 0 { self.column - 1 } else { 0 },
        }
    }
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(Location, char),
    UnexpectedEnd(Location),
}

#[derive(Debug)]
pub enum Token<'a> {
    Open,
    Close,
    Whitespace,
    Boolean(bool),
    Identifier(&'a str),
    Integer(usize),
}

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new<S: Into<String>>(source: S) -> Self {
        Self {
            source: source.into(),
        }
    }

    pub fn tokenize<'a>(&'a self) -> TokenIterator<'a> {
        TokenIterator::new(self)
    }
}

pub struct TokenIterator<'a> {
    lines: Peekable<Lines<'a>>,
    chars: Peekable<Chars<'a>>,
    line: &'a str,
    location: Location,
}

impl<'a> TokenIterator<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        let mut lines = lexer.source.lines().peekable();
        let line = lines.next().unwrap_or("");
        let chars = line.chars().peekable();

        Self {
            lines,
            chars,
            line,
            location: Location::new(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(char) = self.chars.next() {
            self.location.column += 1;
            Some(char)
        } else {
            if let Some(line) = self.lines.next() {
                self.location.line += 1;
                self.location.column = 0;
                self.line = line;
                self.chars = line.chars().peekable();
                self.next_char()
            } else {
                None
            }
        }
    }

    fn parse_whitespace(&mut self) -> Token<'a> {
        loop {
            match self.chars.peek() {
                Some(c) if Rules::is_whitespace(*c) => {
                    self.next_char();
                    continue;
                }
                Some(_) | None => return Token::Whitespace,
            }
        }
    }

    fn parse_boolean(&mut self) -> Result<Token<'a>, LexError> {
        // FIXME: support #true and #false
        match self.next_char() {
            Some('t') => Ok(Token::Boolean(true)),
            Some('f') => Ok(Token::Boolean(false)),
            Some(c) => Err(LexError::UnexpectedChar(self.location.back_one(), c)),
            None => Err(LexError::UnexpectedEnd(self.location)),
        }
    }

    fn parse_identifier(&mut self) -> Token<'a> {
        let start = self.location.column - 1;
        loop {
            match self.chars.peek() {
                Some(c) if Rules::is_identifier(*c) => {
                    self.next_char();
                    continue;
                }
                Some(_) | None => {
                    return Token::Identifier(&self.line[start..self.location.column])
                }
            }
        }
    }

    fn parse_integer(&mut self) -> Token<'a> {
        let start = self.location.column - 1;
        loop {
            match self.chars.peek() {
                Some(c) if Rules::is_integer(*c) => {
                    self.next_char();
                    continue;
                }
                Some(_) | None => {
                    return Token::Integer(
                        self.line[start..self.location.column]
                            .parse::<usize>()
                            .unwrap(),
                    )
                }
            }
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(char) = self.next_char() {
            Some(match char {
                '(' => Ok(Token::Open),
                ')' => Ok(Token::Close),
                '#' => self.parse_boolean(),
                c if Rules::is_whitespace(c) => Ok(self.parse_whitespace()),
                c if Rules::can_start_identifier(c) => Ok(self.parse_identifier()),
                c if Rules::is_integer(c) => Ok(self.parse_integer()),
                c => Err(LexError::UnexpectedChar(self.location.back_one(), c)),
            })
        } else {
            None
        }
    }
}

struct Rules {}

impl Rules {
    const INITIAL: &'static [char] = &[
        '!', '$', '%', '&', '*', '/', ':', '<', '=', '>', '?', '^', '_', '~',
    ];

    const SPEC_SUBSEQUENT: &'static [char] = &['+', '-', '.', '@'];

    fn can_start_identifier(c: char) -> bool {
        c.is_alphabetic() || Self::INITIAL.contains(&c)
        // FIMXE: not part of initial
        || (c == '+' || c == '-')
    }

    fn is_identifier(c: char) -> bool {
        Self::can_start_identifier(c) || c.is_numeric() || Self::SPEC_SUBSEQUENT.contains(&c)
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t'
    }

    fn is_integer(c: char) -> bool {
        c.is_numeric()
    }
}
