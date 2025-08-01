#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).


// Shouldn't take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership (of the ability to mutate via &mut)
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase();
    println!("{data}");
}

fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(data.clone()); // Pass a clone to avoid moving `data`

    string_uppercase(&mut data); // Pass mutable ref to mutate
}
