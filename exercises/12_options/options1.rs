// This function returns how much icecream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no icecream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day < 22 {
        Some(5)
    } else if hour_of_day <= 23 {
        Some(0)
    } else {
        None
    }
    // This is like an if-else statement in Python, but with `Option` type.
    // In Python, it would be like:
    // if hour_of_day < 22:
    //     return 5
    // elif hour_of_day == 22:
    //     return 0
    // else:
    //     return None
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
        let icecreams = maybe_icecream(0).unwrap_or(0); // Use `unwrap_or` to provide a default value if None.
        // This is like `if maybe_icecream(0) is None: return 0 else: return maybe_icecream(0)`
        // in Python.

        assert_eq!(icecreams, 5); // Don't change this line.
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
