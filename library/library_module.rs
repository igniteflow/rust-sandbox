/*
    > rustc --crate-type=lib library_module.rs
 */
pub fn public_function() {
    println!("called library_modules's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}