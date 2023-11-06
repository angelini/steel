use crate::{
    ast::{Ast, Expression},
    value::Value,
};

#[derive(Debug)]
pub enum CompileError {}

pub type ChunkId = usize;

#[derive(Debug)]
pub enum Op {
    Push(usize),
    LoadImmediate(Value),
    LoadVariable,
    Define(String),
    Lookup(String),
    Call(ChunkId),
}

#[derive(Debug)]
pub struct Chunk {
    pub id: usize,
    pub code: Vec<Op>,
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
    chunks: Vec<Chunk>,
}

impl Compiler {
    pub fn new() -> Self {
        Self { chunks: vec![] }
    }

    pub fn compile(mut self, ast: Ast) -> Result<Vec<Chunk>, CompileError> {
        for expression in ast.into_iter() {
            self.compile_expression(expression)?;
        }
        Ok(self.chunks)
    }

    fn compile_expression(&mut self, expression: Expression) -> Result<ChunkId, CompileError> {
        let chunk_id = self.new_chunk_id();

        match expression {
            Expression::Empty => {}
            Expression::Definition(name, expr) => {
                let body = self.compile_expression(*expr)?;
                self.emit(chunk_id, Op::Push(body));
                self.emit(chunk_id, Op::Define(name))
            }
            Expression::Call(identifier, arguments) => {
                let arg_len = arguments.len();
                let arg_chunks = arguments
                    .into_iter()
                    .map(|argument| self.compile_expression(argument))
                    .collect::<Result<Vec<ChunkId>, CompileError>>()?;
                for arg_chunk in arg_chunks {
                    self.emit(chunk_id, Op::Push(arg_chunk))
                }
                self.emit(chunk_id, Op::Lookup(identifier));
                self.emit(chunk_id, Op::Call(arg_len))
            }
            Expression::StaticValue(value) => self.emit(chunk_id, Op::LoadImmediate(value)),
            Expression::Variable(identifier) => {
                self.emit(chunk_id, Op::Lookup(identifier));
                self.emit(chunk_id, Op::LoadVariable)
            }
            _ => {
                println!("compile expression unimplemented: {:?}", expression);
                unimplemented!()
            }
        }

        Ok(chunk_id)
    }

    fn new_chunk_id(&mut self) -> ChunkId {
        let chunk_id = self.chunks.len();
        self.chunks.push(Chunk::new(chunk_id));
        chunk_id
    }

    fn emit(&mut self, chunk_id: ChunkId, op: Op) {
        self.chunks[chunk_id].emit(op)
    }
}
