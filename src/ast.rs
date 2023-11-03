use std::{slice::Iter, vec::IntoIter};

use crate::{
    lexer::{LexError, Token, TokenIterator},
    value::Value,
};

#[derive(Debug)]
pub enum AstError {
    LexError(LexError),
    UnexpectedEnd,
    UnexpectedToken(String),
}

#[derive(Debug)]
pub enum Expression {
    Empty,
    Call(String, Vec<Expression>),
    Definition(String, Box<Expression>),
    StaticValue(Value),
    Variable(String),
}

#[derive(Debug)]
pub struct Ast {
    exprs: Vec<Expression>,
}

impl Ast {
    fn new() -> Self {
        Self { exprs: vec![] }
    }

    pub fn iter(&self) -> Iter<Expression> {
        self.exprs.iter()
    }

    pub fn into_iter(self) -> IntoIter<Expression> {
        self.exprs.into_iter()
    }
}

pub struct AstBuilder<'a> {
    tokens: TokenIterator<'a>,
}

impl<'a> AstBuilder<'a> {
    pub fn new(tokens: TokenIterator<'a>) -> Self {
        Self { tokens }
    }

    pub fn build(&mut self) -> Result<Ast, AstError> {
        let mut ast = Ast::new();
        loop {
            match self.tokens.next() {
                Some(Ok(token)) => match token {
                    Token::Open => {
                        let tokens = self.read_until_matching_close()?;
                        ast.exprs.push(parse_expression(&tokens)?)
                    }
                    Token::Close => return Err(AstError::UnexpectedToken(")".to_string())),
                    Token::Whitespace => continue,
                    _ => {
                        println!("ast build token not implemented: {:?}", token);
                        unimplemented!()
                    }
                },
                Some(Err(lex_error)) => return Err(AstError::LexError(lex_error)),
                None => return Ok(ast),
            }
        }
    }

    fn read_until_matching_close(&mut self) -> Result<Vec<Token<'a>>, AstError> {
        let mut depth = 0;
        let mut buffer = vec![Token::Open];
        loop {
            match self.tokens.next() {
                Some(Ok(token)) => match token {
                    Token::Open => {
                        depth += 1;
                        buffer.push(token);
                    }
                    Token::Close => {
                        buffer.push(token);
                        if depth == 0 {
                            return Ok(buffer);
                        } else {
                            depth -= 1;
                        }
                    }
                    Token::Whitespace => {}
                    _ => buffer.push(token),
                },
                Some(Err(lex_error)) => return Err(AstError::LexError(lex_error)),
                None => return Err(AstError::UnexpectedEnd),
            }
        }
    }
}

fn parse_expression<'a>(tokens: &[Token<'a>]) -> Result<Expression, AstError> {
    match &tokens[..] {
        [Token::Boolean(bool)] => Ok(Expression::StaticValue(Value::Boolean(*bool))),
        [Token::Integer(number)] => Ok(Expression::StaticValue(Value::Number(*number))),
        [Token::Identifier(name)] => Ok(Expression::Variable(name.to_string())),
        [Token::Open, Token::Close] => Ok(Expression::Empty),
        [Token::Open, Token::Identifier("define"), Token::Identifier(ident), .., Token::Close] => {
            Ok(Expression::Definition(
                ident.to_string(),
                Box::new(parse_expression(&tokens[3..tokens.len() - 1])?),
            ))
        }
        [Token::Open, Token::Identifier(operator), .., Token::Close] => Ok(Expression::Call(
            operator.to_string(),
            parse_expressions(&tokens[2..tokens.len() - 1])?,
        )),
        _ => {
            println!("parse expression unimplemented: {:?}", tokens);
            unimplemented!()
        }
    }
}

fn parse_expressions<'a>(tokens: &[Token<'a>]) -> Result<Vec<Expression>, AstError> {
    let mut expressions = vec![];
    for i in 0..tokens.len() {
        match &tokens[i] {
            Token::Open => expressions.push(parse_expression(
                &tokens[i..index_of_matching_close(&tokens[i..])? + 1],
            )?),
            Token::Close => return Err(AstError::UnexpectedEnd),
            _ => expressions.push(parse_expression(&tokens[i..i + 1])?),
        }
    }
    Ok(expressions)
}

fn index_of_matching_close<'a>(tokens: &[Token<'a>]) -> Result<usize, AstError> {
    let mut depth = 0;
    for (idx, token) in tokens.iter().enumerate() {
        match token {
            Token::Open => {
                depth += 1;
            }
            Token::Close => {
                if depth == 0 {
                    return Ok(idx);
                } else {
                    depth -= 1;
                }
            }
            _ => continue,
        }
    }
    Err(AstError::UnexpectedEnd)
}
