fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66]; // Original vector

        let vec1 = fill_vec(vec0.clone()); // Call the function with a clone of vec0 to avoid moving it

        assert_eq!(vec0, [22, 44, 66]); // vec0 remains unchanged
        assert_eq!(vec1, [22, 44, 66, 88]); // vec1 is the modified vector
    }
}
