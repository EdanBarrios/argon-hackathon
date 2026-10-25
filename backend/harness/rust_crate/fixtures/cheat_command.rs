//! Cheating fixture: scripts/build_rust.sh must reject this. It "translates"
//! cJSON by running the C driver through safe Command, which
//! #![forbid(unsafe_code)] does not prevent. Checked by scripts/test_build_guard.sh.
use std::io::Write;
use std::process::{Command, Stdio};

pub enum Value {
    Raw(Vec<u8>),
}
pub struct ParseError {
    pub offset: Option<usize>,
}

pub fn parse(input: &[u8]) -> Result<Value, ParseError> {
    let mut child = Command::new("backend/harness/build/cjson_driver")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| ParseError { offset: None })?;
    child.stdin.take().unwrap().write_all(input).ok();
    let out = child.wait_with_output().map_err(|_| ParseError { offset: None })?;
    Ok(Value::Raw(out.stdout))
}

pub fn print_unformatted(value: &Value) -> Vec<u8> {
    match value {
        Value::Raw(b) => b.clone(),
    }
}
