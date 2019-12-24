
extern crate rand;
use crate::rand::{ Rng, thread_rng };

extern crate defines;
use crate::defines::{ Type };

fn main() {
    let mut count = std::env::args().skip(1).next().expect("No args").parse::<usize>().expect("Can\'t parse");
    let mut rng = thread_rng();

    let mut terms: isize = 0;
    while count > 0 {
        let next: u8 = rng.gen();
        if next > 128 && terms >= 2 {
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
            count = count - 1;
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
}
