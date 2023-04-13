#![allow(dead_code)]

#[derive(Debug)]
struct Dog {
    name: String,
    breed: String,
    age: u8,
}

fn main() {
    let name = String::from("Doge");
    let breed = String::from("Shiba Inu");
    let age = 10;
    let dog = Dog{name, breed, age};
    println!("{dog:?}");
    println!("Name: {0} Age: {1}", dog.name, dog.age);

    // Output
    // Dog { name: "Doge", breed: "Shiba Inu", age: 10 }
    // Name: Doge Age: 10
}