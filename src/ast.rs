use crate::{lexer, value};

#[derive(Debug)]
pub enum AstError {
    LexError(lexer::LexError),
    UnexpectedEnd,
    UnexpectedToken(String),
}

#[derive(Debug)]
enum Expression {
    StaticValue(value::Value),
}

#[derive(Debug)]
struct SExprNode {
    car: String,
    cdr: Vec<Box<SExprNode>>,
}

#[derive(Debug)]
struct Definition {
    identifier: String,
    expression: Expression,
}

#[derive(Debug)]
pub struct Ast {
    definitions: Vec<Definition>,
}

impl Ast {
    fn new() -> Self {
        Self {
            definitions: vec![],
        }
    }
}

pub struct AstBuilder<'a> {
    tokens: lexer::TokenIterator<'a>,
}

impl<'a> AstBuilder<'a> {
    pub fn new(tokens: lexer::TokenIterator<'a>) -> Self {
        Self { tokens }
    }

    pub fn build(&mut self) -> Result<Ast, AstError> {
        let mut ast = Ast::new();
        loop {
            match self.tokens.next() {
                Some(Ok(token)) => match token {
                    lexer::Token::Open => {
                        let tokens = self.read_until_matching_close()?;
                        let definition = parse_tokens(tokens)?;
                        ast.definitions.push(definition)
                    }
                    lexer::Token::Close => return Err(AstError::UnexpectedToken(")".to_string())),
                    _ => unimplemented!(),
                },
                Some(Err(lex_error)) => return Err(AstError::LexError(lex_error)),
                None => return Ok(ast),
            }
        }
    }

    fn read_until_matching_close(&mut self) -> Result<Vec<lexer::Token<'a>>, AstError> {
        let mut depth = 0;
        let mut buffer = vec![];
        loop {
            match self.tokens.next() {
                Some(Ok(token)) => match token {
                    lexer::Token::Open => {
                        depth += 1;
                        buffer.push(token);
                    }
                    lexer::Token::Close => {
                        if depth == 0 {
                            return Ok(buffer);
                        } else {
                            depth -= 1;
                            buffer.push(token);
                        }
                    }
                    lexer::Token::Whitespace => {}
                    _ => buffer.push(token),
                },
                Some(Err(lex_error)) => return Err(AstError::LexError(lex_error)),
                None => return Err(AstError::UnexpectedEnd),
            }
        }
    }
}

fn parse_tokens<'a>(tokens: Vec<lexer::Token<'a>>) -> Result<Definition, AstError> {
    match &tokens[..] {
        [lexer::Token::Identifier("define"), lexer::Token::Identifier(ident), expr] => {
            Ok(Definition {
                identifier: ident.to_string(),
                expression: parse_expression(expr)?,
            })
        }
        _ => {
            println!("parse tokens unimplemented: {:?}", tokens);
            unimplemented!()
        }
    }
}

fn parse_expression<'a>(token: &lexer::Token<'a>) -> Result<Expression, AstError> {
    match token {
        lexer::Token::Integer(number) => Ok(Expression::StaticValue(value::Value::Number(*number))),
        _ => {
            println!("parse expr unimplemented: {:?}", token);
            unimplemented!()
        }
    }
}
