use std::collections::HashMap;

use crate::{compiler::Chunk, value::Value};

#[derive(Debug)]
pub enum VmError {}

pub struct Vm {
    globals: HashMap<String, Value>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }

    pub fn execute(&mut self, chunk: &Chunk) -> Result<(), VmError> {
        unimplemented!()
    }
}
