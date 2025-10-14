struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    let red = ColorRegularStruct{red: 255, green: 0, blue: 0};
    println!("{}, {}, {}", red.red, red.green, red.blue);

    let green = ColorTupleStruct(0, 255, 0);
    println!("{}, {}, {}", green.0, green.1, green.2);

    let unit = UnitStruct;
    let unit_message = format!("{unit:?}");
    println!("{unit_message}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        let green = ColorRegularStruct {
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
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        let unit_struct = UnitStruct;

        let message = format!("{unit_struct:?}s are fun!");
        assert_eq!(message, "UnitStructs are fun!");
    }
}
