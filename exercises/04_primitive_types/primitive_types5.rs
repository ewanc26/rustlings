fn main() {
    let cat = ("Furry McFurson", 3.5);
    // I know it's just a cat but I immediately thought of that one meme where it shows Rust devs (Rustaceans?) being furries, lol.
    // It's not wrong about me though, to be fair. I am a furry if you have to call me that, though i distance myself from the term "furry" nowadays as it has a lot of negative connotations.
    // I just like cute animals, and I like to write with them in mind, so I guess I am a furry. ugh. anyway, let's get back to the code.

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;

    let (name, age) = cat; // Destructuring the tuple into name and age

    println!("{name} is {age} years old");
}
