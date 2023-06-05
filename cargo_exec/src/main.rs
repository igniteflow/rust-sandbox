use std::env;

fn main() {
    /*
        ❯ export EXAMPLE='Hello, world'
        ❯ cargo build
        Compiling cargo_exec v0.1.0 (/Users/ptysoe/Projects/personal/rust-sandbox/cargo_exec)
            Finished dev [unoptimized + debuginfo] target(s) in 0.86s
        ❯ export EXAMPLE="foobar" && ./target/debug/cargo_exec
        EXAMPLE: "foobar"
        Value: "foobar"
     */
    let key = "EXAMPLE";

    // print the value if exists else print error
    match env::var(key) {
        Ok(val) => println!("{key}: {val:?}"),
        Err(e) => println!("Error reading environment var {key}: {e}"),
    }

    // assign to var and then print - note does not handle error
    let env_val = env::var(key).unwrap();
    println!("Value: {env_val:?}");
}
