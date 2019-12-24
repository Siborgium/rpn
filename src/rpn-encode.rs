
use std::fs::File;
use std::io::{ BufReader, BufRead, Write };

extern crate defines;
use crate::defines::{ Type, DEFINES };

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let reader = BufReader::new(handle);    

    let mut file = File::create("input.txt")?;
    for v in reader.split(b' ') {
        let v = v.expect("Failed to split");
        let s = std::str::from_utf8(&v).expect("Not utf8").trim();
        if !s.is_empty() {
            if let Some((_, r)) = ["+", "-", "*"].iter().zip(DEFINES.iter()).find(|(e, _)| *e == &s) {
                file.write(&r.to_ne_bytes()[..])?;
            } else {
                file.write(&s.parse::<Type>().expect("Can't parse").to_ne_bytes()[..])?;
            }
        }
    }
    Ok(())
}
