
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate defines;
use crate::defines::{ Type, DEFINES, STACK_SIZE, CHUNK_SIZE, SIZE };

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, file);

    let mut stack = Vec::with_capacity(STACK_SIZE);

    let plus = DEFINES[0].to_ne_bytes();
    let minus = DEFINES[1].to_ne_bytes();
    let multiply = DEFINES[2].to_ne_bytes();

    'running: loop {
        let buf = reader.fill_buf()?;
        let consumed = buf.len();
        let chunks = buf.chunks_exact(SIZE);
        for chunk in chunks {
            let raw = unsafe { *(chunk.as_ptr() as *const [u8; SIZE]) };
            
            let is_plus = raw == plus;
            let is_minus = raw == minus;
            let is_multiply = raw == multiply;
            if is_plus || is_minus || is_multiply {
                let b = stack.pop().expect("Pop failed");                
                let a = stack.pop().expect("Pop failed");
                let c = match (is_plus, is_minus, is_multiply) {
                    (true, _, _) => a + b,
                    (_, true, _) => a - b,
                    (_, _, true) => a * b,
                    _ => unsafe { std::hint::unreachable_unchecked() }
                };
                stack.push(c);
            } else {
                let v = Type::from_ne_bytes(raw);
                stack.push(v);
            }
        }
        if buf.is_empty() {
            break 'running;
        }
        reader.consume(consumed);
    }
    assert!(stack.len() == 1);
    println!("{}", stack[0]);
    Ok(())
}
