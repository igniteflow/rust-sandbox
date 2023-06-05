/*
    rustc executable.rs --extern rary=liblibrary_module.rlib && ./executable
 */
fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}