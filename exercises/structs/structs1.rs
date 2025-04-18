// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

struct ColorClassicStruct {
    // TODO: Something goes here
    red: i32,
    green: i32,
    blue: i32,
}

impl ColorClassicStruct {
    fn new(r: i32, g: i32, b: i32) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
}

struct ColorTupleStruct(i32, i32, i32);

impl ColorTupleStruct {
    fn new(r: i32, g: i32, b: i32) -> Self {
        ColorTupleStruct(r, g, b)
    }
}

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct::new(0, 255, 0);
        let g = ColorClassicStruct {
            red: 1,
            green: 2,
            blue: 3,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct::new(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
