/*
    ❯ rustc bindings.rs && ./bindings
    Outer scope before 1
    Inner scope I am your inner self
    Outer scope after 1
    Mutant Ahoy
    Mutant Ahoy
    Mutant there
 */
fn main() {
    shadowing();
    freezing();
}

fn shadowing() {
    let variable = 1;
    println!("Outer scope before {}", variable);

    // block
    {
        let variable = "I am your inner self";
        println!("Inner scope {}", variable);
    }
    println!("Outer scope after {}", variable);
}

fn freezing() {
    // mutable
    let mut mutant_str = "Ahoy!";

    {
        // not mutable
        let copy_mutant = mutant_str;
        println!("I am not a mutant {}", copy_mutant);

        //copy_mutant = "foobar"; if uncommented this will raise an error
    }
    println!("Mutant {}", mutant_str);
    mutant_str = "there";
    println!("Mutant {}", mutant_str);
}
