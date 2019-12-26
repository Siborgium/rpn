
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
            
            let is_op = ((raw == plus) as u8) // 0, 1, 2, 4
                + (((raw == minus) as u8) << 1)
                + (((raw == multiply) as u8) << 2);

            if is_op == 0 {
                stack.push(Type::from_ne_bytes(raw));
            } else {
                let b = if let Some(t) = stack.pop() {
                    t
                } else {
                    unsafe { std::hint::unreachable_unchecked() }
                };
                let a = if let Some(t) = stack.pop() {
                    t
                } else {
                    unsafe { std::hint::unreachable_unchecked() }
                };
                let c = match is_op {
                    1 => a + b,
                    2 => a - b,
                    4 => a * b,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };
                stack.push(c);
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
