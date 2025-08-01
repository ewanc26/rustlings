// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));

    // that certainly won't get confusing...

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]); // slice of a String -> &str

    string_slice("  hello there ".trim()); // trim returns &str

    string("Happy Monday!".replace("Mon", "Tues")); // replace returns a new String

    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // to_lowercase returns a new String... also, ew that connotation is gross.
}