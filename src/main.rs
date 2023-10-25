use ast::AstBuilder;

mod ast;
mod lexer;
mod value;

fn main() {
    let lexer = lexer::Lexer::new("(define x (+ 1 #t))");

    println!("tokens: [");
    for token in lexer.tokenize() {
        match token {
            Ok(token) => println!("    {:?},", token),
            Err(err) => {
                println!("lex error: {:?}", err);
                return;
            }
        }
    }

    println!("]\n");

    let mut builder = AstBuilder::new(lexer.tokenize());
    match builder.build() {
        Ok(ast) => println!("ast: {:?}", ast),
        Err(err) => println!("ast error: {:?}", err),
    }
}
