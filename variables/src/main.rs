// Naming conventions for constants is all uppercase and underscores between the words
// const HERE_IS_A_CONST: u32 = 100;

fn main() {
    new_function();
}

fn new_function() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
