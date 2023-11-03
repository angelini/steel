use ast::{Ast, AstBuilder, AstError};
use compiler::{Chunk, CompileError, Compiler};
use lexer::{LexError, Lexer};
use vm::Vm;

mod ast;
mod compiler;
mod lexer;
mod native;
mod value;
mod vm;

fn lex<S: Into<String>>(source: S, debug: bool) -> Result<Lexer, LexError> {
    let lexer = Lexer::new(source);

    if debug {
        println!("tokens: [");
        for token in lexer.tokenize() {
            match token {
                Ok(token) => println!("    {:?},", token),
                Err(err) => {
                    return Err(err);
                }
            }
        }
        println!("]\n");
    }

    Ok(lexer)
}

fn parse(lexer: Lexer, debug: bool) -> Result<Ast, AstError> {
    let mut builder = AstBuilder::new(lexer.tokenize());
    let ast = builder.build()?;

    if debug {
        println!("ast: {:?}", ast)
    }

    Ok(ast)
}

fn compile(ast: Ast, debug: bool) -> Result<Vec<Chunk>, CompileError> {
    let mut compiler = Compiler::new();
    let chunks = compiler.compile(ast)?;

    if debug {
        for chunk in chunks.iter() {
            println!("chunk: {:?}", chunk)
        }
    }

    Ok(chunks)
}

fn execute<S: Into<String>>(source: S, debug: bool) {
    let mut vm = Vm::new();

    match lex(source, debug) {
        Ok(lexer) => match parse(lexer, debug) {
            Ok(ast) => match compile(ast, debug) {
                Ok(chunks) => {
                    for chunk in chunks.iter() {
                        if let Err(err) = vm.execute(&chunk) {
                            println!("exec error: {:?}", err)
                        }
                    }
                }
                Err(err) => {
                    println!("compile error: {:?}", err)
                }
            },
            Err(err) => {
                println!("ast error: {:?}", err)
            }
        },
        Err(err) => {
            println!("lex error: {:?}", err)
        }
    }
}

fn main() {
    let source = "
        (define x (+ 1 2))
        (display x)
    ";

    execute(source, true);
}
