struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?

    red: u8,
    green: u8,
    blue: u8,
}

// This is a tuple struct, which is a struct that has no named fields but has a fixed number of fields. Weird concept but okay.
struct ColorTupleStruct(
    /* TODO: Add the fields that the test `tuple_structs` expects */
    u8, // red
    u8, // green
    u8, // blue
);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        // let green =

        let green = super::ColorRegularStruct { // red: 0, green: 255, blue: 0 };
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        // let green =

        let green = super::ColorTupleStruct(0, 255, 0); // green :3c i like forest green

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        // let unit_struct =

        let unit_struct = super::UnitStruct; // Unit structs are instantiated without any fields.
        // i'm not sure why they exist but okay. i assume they are useful for some reason.

        let message = format!("{unit_struct:?}s are fun!"); // Using the debug format to print the unit struct.

        assert_eq!(message, "UnitStructs are fun!");
    }
}
