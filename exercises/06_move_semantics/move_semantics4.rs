fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);

        // Put `z` after `y` to avoid borrowing `x` while it's already borrowed as mutable.
        let z = &mut x; 
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
