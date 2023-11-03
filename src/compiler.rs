use crate::{
    ast::{Ast, Expression},
    native::NativeFn,
    value::Value,
};

#[derive(Debug)]
pub enum CompileError {}

enum Location {
    Value,
    Variable,
    FnChunk,
}

#[derive(Debug)]
enum Op {
    Define(String),
    ImmediateValue(Value),
    CallNative(NativeFn),
}

#[derive(Debug)]
pub struct Chunk {
    id: usize,
    code: Vec<Op>,
}

impl Chunk {
    fn new(id: usize) -> Self {
        Self { id, code: vec![] }
    }

    fn emit(&mut self, op: Op) {
        self.code.push(op)
    }
}

pub struct Compiler {
    chunk_id: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self { chunk_id: 0 }
    }

    pub fn compile(&mut self, ast: Ast) -> Result<Vec<Chunk>, CompileError> {
        ast.into_iter()
            .map(|expr| Ok(self.compile_expression(expr)))
            .collect()
    }

    fn compile_expression(&mut self, expression: Expression) -> Chunk {
        let mut chunk = self.new_chunk();
        match expression {
            Expression::Empty => {}
            Expression::Definition(name, expr) => {
                // FIXME: compile body
                chunk.emit(Op::Define(name))
            }
            _ => {
                println!("compile expression unimplemented: {:?}", expression);
                unimplemented!()
            }
        }
        chunk
    }

    fn new_chunk(&mut self) -> Chunk {
        self.chunk_id += 1;
        Chunk::new(self.chunk_id)
    }
}
