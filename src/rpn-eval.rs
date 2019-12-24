
use std::fs::File;

extern crate memmap;
use memmap::{ Mmap, MmapOptions };

mod defines;
use defines::{ Type, DEFINES, STACK_SIZE, CHUNK_SIZE, SIZE };

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let map = unsafe { Mmap::map(&file)? };

    let mut stack = MmapOptions::new().stack().len(STACK_SIZE).map_anon()?;
    let mut stack_len = 0;
    let begin = stack.as_mut_ptr() as *mut Type;

    let plus = DEFINES[0].to_ne_bytes();
    let minus = DEFINES[1].to_ne_bytes();
    let multiply = DEFINES[2].to_ne_bytes();

    for chunk in map.chunks(CHUNK_SIZE) {
        for bytes in chunk.chunks(SIZE) {
            let raw = unsafe { *(bytes.as_ptr() as *const [u8; SIZE]) };
            
            let is_plus = raw == plus;
            let is_minus = raw == minus;
            let is_multiply = raw == multiply;
            if is_plus || is_minus || is_multiply {
                let b = unsafe { *(begin.offset(stack_len - 1) as *const Type) };
                let a = unsafe { *(begin.offset(stack_len - 2) as *const Type) };
                let c = match (is_plus, is_minus, is_multiply) {
                    (true, _, _) => a + b,
                    (_, true, _) => a - b,
                    (_, _, true) => a * b,
                    _ => unsafe { std::hint::unreachable_unchecked() }
                };
                unsafe { begin.offset(stack_len - 2).write(c) };
                stack_len = stack_len - 1;             
            } else {
                let v = Type::from_ne_bytes(unsafe { *(bytes.as_ptr() as *const [u8; SIZE]) });
                unsafe { begin.offset(stack_len).write(v) };
                stack_len = stack_len + 1;
            }
        }
    }
    println!("{}", unsafe { *begin });
    Ok(())
}
