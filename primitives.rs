use std::mem;

// rustc primitives.rs && ./primitives
fn main() {
    // string
    let subject = "igniteflow";
    println!("Hello, {subject}!");

    // tuple
    let test_tuple = (1, 2, 3);
    println!("Tuple: {test_tuple:?}");
    println!("Reversed tuple: {:?}", reverse(test_tuple));

    // array - fixed size
    let test_array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("Array: {test_array:?}");
    println!("Reversed array: {:?}", reverse_array(test_array));

    // array - memory used (bytes)
    println!("Bytes: {:?}", mem::size_of_val(&test_array));

    // slice - size not known at compile time
    let test_slice = &test_array[0..2];
    println!("Slice: {:?}", test_slice);
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
