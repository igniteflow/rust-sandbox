use std::env;

fn main() {
    /*
        ❯ export EXAMPLE='Hello, world'
        ❯ cargo build
        Compiling cargo_exec v0.1.0 (/Users/ptysoe/Projects/personal/rust-sandbox/cargo_exec)
            Finished dev [unoptimized + debuginfo] target(s) in 0.86s
        ❯ ./target/debug/cargo_exec
        EXAMPLE: "Hello, world"
     */
    let key = "EXAMPLE";
    match env::var(key) {
        Ok(val) => println!("{key}: {val:?}"),
        Err(e) => println!("Error reading environment var {key}: {e}"),
    }
}
