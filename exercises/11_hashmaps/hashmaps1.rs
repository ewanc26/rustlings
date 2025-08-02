// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.

use std::collections::HashMap;
// `import std.collections.hashmap` in Python, i guess?
// HashMap is a collection of key-value pairs, where each key is unique and
// maps to a value. It's similar to a dictionary in Python.

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    // let mut basket =

    let mut basket: HashMap<String, u32> = HashMap::new(); // that was easy. 

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket.
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 1);
    basket.insert(String::from("grape"), 6);
    // I don't like bananas so i went with apples, mangoes, and grapes instead.

    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
