/*
 ❯ rustc ownership.rs && ./ownership
 hello hello
 32 32
 */
fn main() {
    // https://doc.rust-lang.org/1.30.0/book/second-edition/ch04-01-what-is-ownership.html

    // Variables on the heap call clone not copy
    // Trying to "assign" s1 to s2, will move the value, invalidating s1
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);

    // Literals are stored on the stack.  These values can be copied without
    // the first value being invalidated or having to call clone.
    let l1: u32 = 32;
    let l2 = l1;
    println!("{} {}", l1, l2);

}