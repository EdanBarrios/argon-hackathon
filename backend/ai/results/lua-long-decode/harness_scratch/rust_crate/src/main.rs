#![forbid(unsafe_code)]
use std::io::{Read, Write};

mod lua;

fn main() {
    let mut input = Vec::new();
    if std::io::stdin().read_to_end(&mut input).is_err() {
        std::process::exit(2);
    }
    // The parser and C-level calls may recurse up to LUAI_MAXCCALLS (200) levels.
    let out = std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(move || lua::run(&input))
        .ok()
        .and_then(|h| h.join().ok());
    match out {
        Some(bytes) => {
            let mut stdout = std::io::stdout().lock();
            let _ = stdout.write_all(&bytes);
            let _ = stdout.flush();
        }
        None => std::process::exit(101),
    }
}
