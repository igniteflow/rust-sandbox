/*
    ❯ rustc conversions.rs && ./conversions
    My number is MegaDrive { value: 30 }
    My number is MegaDrive { value: 5 }
    Do you know how to get to Bells Canyon?
 */
use std::convert::From;

#[allow(dead_code)]
#[derive(Debug)]
struct MegaDrive {
    value: i16,
}

impl From<i16> for MegaDrive {
    fn from(item: i16) -> Self {
        MegaDrive { value: item }
    }
}

fn from() {
    let num = MegaDrive::from(30);
    println!("My number is {:?}", num);
}

fn into() {
    // the compiler infers into using "From", but we must specify
    // MegaDrive so that it knows what type to convert into
    let int = 5;
    let num: MegaDrive = int.into();
    println!("My number is {:?}", num);
}

fn str_to_string() {
    // convert a str into a String
    let a_str = "Do you know how to get to Bells Canyon?";
    let a_string = String::from(a_str);
    println!("{}", a_string);
}

fn main() {
    from();
    into();
    str_to_string();
}

