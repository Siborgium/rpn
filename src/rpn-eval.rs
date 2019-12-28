
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate defines;
use crate::defines::{ Raw, from_bits, to_bits, DEFINES, STACK_SIZE, CHUNK_SIZE, SIZE };

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, file);

    let mut stack = Vec::with_capacity(STACK_SIZE);

    let plus = to_bits(DEFINES[0]);
    let minus = to_bits(DEFINES[1]);
    let multiply = to_bits(DEFINES[2]);

    'running: loop {
        let buf = if let Ok(buf) = reader.fill_buf() {
            buf
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        };

        let consumed = buf.len();
        for chunk in buf.chunks_exact(SIZE) {
            let raw = unsafe { *(chunk.as_ptr() as *const Raw) };
            
            let is_op = ((raw == plus) as u8) // 0, 1, 2, 4
                + (((raw == minus) as u8) << 1)
                + (((raw == multiply) as u8) << 2);

            
            let c;
            if is_op == 0 {
                c = from_bits(raw)
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
                c = match is_op {
                    1 => a + b,
                    2 => a - b,
                    4 => a * b,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };
            }
            stack.push(c);
        }
        if buf.is_empty() {
            break 'running;
        }
        reader.consume(consumed);
    }
    assert!(stack.len() == 1);
    println!("{:.}", stack[0]);
    Ok(())
}
