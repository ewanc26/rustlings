// TODO: Fix the compiler error.
fn main() {
    let mut x = 3; // mut is the missing keyword, it allows x to be mutable
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
