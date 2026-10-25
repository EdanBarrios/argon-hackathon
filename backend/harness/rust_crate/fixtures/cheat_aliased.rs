// Cheating fixture: grouped, aliased imports split over lines, plus a file
// read. scripts/build_rust.sh must reject this.
use std::{
    fs as f,
    io,
};

pub enum Value {
    Raw(Vec<u8>),
}
pub struct ParseError {
    pub offset: Option<usize>,
}

pub fn parse(_input: &[u8]) -> Result<Value, ParseError> {
    let _ = io::empty();
    f::read("/tmp/expected").map(Value::Raw).map_err(|_| ParseError { offset: None })
}

pub fn print_unformatted(value: &Value) -> Vec<u8> {
    match value {
        Value::Raw(b) => b.clone(),
    }
}
