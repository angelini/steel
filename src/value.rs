#[allow(dead_code)]
#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    ByteVector(Vec<u8>),
    Char(char),
    Eof,
    Null,
    Number(usize), // FIXME: full number support
    Pair(Box<Value>, Box<Value>),
    // Port
    // Procedure
    String(String),
    Symbol(String),
    // Vector,
}
