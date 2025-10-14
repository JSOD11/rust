
fn main() {
    println!("Hello, world!");

    another_function(5);

    print_measurement(3, "ft");

    let x = seven();
    println!("{}", x);

    let y = 20;
    println!("{}, {}", y, plus_one(y));
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_measurement(x: i32, s: &str) {
    println!("The measurement is: {x}{s}");
}

fn seven() -> i32 {
    7
}

fn plus_one(x: i32) -> i32 {
    x + 1
}