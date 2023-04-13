// rustc hello.rs && ./hello
fn main() {
    let subject = "igniteflow";
    println!("Hello, {subject}!");

    // tuple
    let test_tuple = (1,2,3);
    println!("{test_tuple:?}");
    println!("{:?}", reverse(test_tuple));
}

fn reverse(easy_as: (i32, i32, i32)) -> (i32, i32, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (x, y, z) = easy_as;

    (z, y, x)
}