// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

pub mod delicious_snacks { // Publicity added to the module.
    // TODO: Add the following two `use` statements after fixing them.

    // this syntax is weird.
    // anyway, this is how you can import a module and rename it.
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    pub mod fruits { // Publicity added to the module.
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies { // Publicity added to the module.
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
