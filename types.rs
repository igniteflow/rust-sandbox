/*
    ❯ rustc types.rs && ./types
    65.4321
    65
    A
 */

fn main() {
    casting();
}

fn casting() {
    let decimal: f32 = 65.4321;
    println!("{}", decimal);

    let integer: u8 = decimal as u8;
    println!("{}", integer);

    let character: char = integer as char;
    println!("{}", character);
}