
use std::fs::File;
use std::io::Write;

mod defines;
use defines::{ Type, DEFINES };

fn main() -> std::io::Result<()> {
    let ss = std::env::args().skip(1).next().expect("No arguments");
    let ss = ss.as_str();

    let mut file = File::create("input.txt")?;
    for s in ss.split(' ') {
        let s = s.trim();
        if let Some((_, r)) = ["+", "-", "*"].iter().zip(DEFINES.iter()).find(|(e, _)| *e == &s) {
            file.write(&r.to_ne_bytes()[..])?;
        } else {
            file.write(&s.parse::<Type>().expect("Can't parse").to_ne_bytes()[..])?;
        }
    }
    Ok(())
}
