fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;

    // Never used a vector macro before. Honestly thought vectors were just arrays with dynamic size
    // used for things like SVGs and stuff.
    // I guess I was wrong, but I don't mind. I like learning new things.
 
    let v = vec![10, 20, 30, 40]; // Vector created with the same elements as the array

    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
