// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.

// Generic wrapper that can hold any type `T`
struct Wrapper<T> {
    value: T,
}

// Implement a constructor for Wrapper
impl<T> Wrapper<T> {
    fn new(value: T) -> Self { // Create a new Wrapper instance with the given value
        Wrapper { value }
    }
}

fn main() {
    let int_wrapper = Wrapper::new(42);
    let str_wrapper = Wrapper::new("Foo"); // Now we can wrap a string as well!

    println!("int_wrapper: {}", int_wrapper.value);
    println!("str_wrapper: {}", str_wrapper.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
