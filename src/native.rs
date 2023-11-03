use crate::value::Value;

#[derive(Debug)]
pub enum NativeFn {
    Add,
    Print,
}

pub fn execute_native(func: NativeFn, args: &[Value]) -> Value {
    unimplemented!()
}
