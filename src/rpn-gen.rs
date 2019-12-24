
extern crate rand;
use crate::rand::{ Rng, thread_rng };

extern crate defines;
use crate::defines::{ Type };

fn main() {
    let count = std::env::args().skip(1).next().expect("No args").parse::<isize>().expect("Can\'t parse");
    let mut rng = thread_rng();

    let mut terms: isize = 0;
    for _ in 0..count {
        if terms >= 2 {
            let next: u8 = rng.gen();
            if next > 128 {
                let op = match next % 3 {
                    0 => '+',
                    1 => '-',
                    2 => '*',
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };
                print!("{} ", op);
                terms = terms - 1;
            } else {
                print!("{} ", rng.gen::<Type>().abs());
                terms = terms + 1;
            }
        } else {
            print!("{} ", rng.gen::<Type>().abs());  
            terms = terms + 1;
        }
    }
    while terms >= 2 {
        let op = match rng.gen::<u8>() % 3 {
            0 => '+',
            1 => '-',
            2 => '*',
            _ => unsafe { std::hint::unreachable_unchecked() },
        };
        print!("{} ", op);
        terms = terms - 1;
    }
    while terms < 1 {
        print!("{} ", rng.gen::<Type>().abs());  
        terms = terms + 1;
    }
}
