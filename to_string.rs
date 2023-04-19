/*
    ❯ rustc to_string.rs && ./to_string
    Number of toys made 25
    370
 */
use std::fmt;

struct ChristmasElf {
    toys_made: i8
}

impl fmt::Display for ChristmasElf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number of toys made {}", self.toys_made)
    }
}

fn parse_string() {
    let num_days: i16 = "365".parse().unwrap();
    println!("{}", num_days + 5);
}

fn main() {
    let elf = ChristmasElf{ toys_made: 25 };
    println!("{}", elf.to_string());
    parse_string();
}