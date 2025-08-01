// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string() // This is how you convert a string literal to a String in Rust.

    // I guess I could have also used String::from("blue") but this is more concise.
    // I wonder if there's a way to do this without using the to_string() method.

    // either way, that was easy.
    // I guess I just had to remember that Rust has a to_string() method for string
    // literals that converts them to a String type.

    // I'm not using the Rustlang book for this, I'm just using my own knowledge
    // of other programming languages and the Rustlings exercises.
    // I hope that's okay.
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
