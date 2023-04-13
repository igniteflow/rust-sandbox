// rustc hello.rs && ./hello
fn main() {
    // string
    let subject = "igniteflow";
    println!("Hello, {subject}!");

    // tuple
    let test_tuple = (1, 2, 3);
    println!("{test_tuple:?}");
    println!("{:?}", reverse(test_tuple));

    // array
    let test_array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("{test_array:?}");
    println!("{:?}", reverse_array(test_array));
}

fn reverse(easy_as: (i32, i32, i32)) -> (i32, i32, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (x, y, z) = easy_as;

    (z, y, x)
}

fn reverse_array(easy_as: [i32; 6]) -> [i32; 6] {
    let [a, b, c, x, y, z] = easy_as;

    [z, y, x, c, b, a]
}
