
fn main() {

    println!();

    let number = 3;

    if number < 5 {
        println!("Condition is true!");
    } else {
        println!("Condition is false!");
    }

    let cond = true;
    let x = if cond { 5 } else { 10 };
    println!("The value of x is: {}", x);

    println!();

    let mut i = 0;
    let out = loop {
        println!("loop");
        if i == 10 {
            break i * 2
        }
        i += 1;
    };

    println!("The value of out is: {}\n", out);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break
            }
            if count == 2 {
                break 'counting_up
            }
            remaining -= 1
        }
        count += 1
    }

    println!("\nEnd count = {count}\n");

    let mut n = 5;
    while n > 0 {
        println!("n = {n}");
        n -= 1;
    }
    println!("\nn = {n}\n");

    let arr = [1, 2, 3, 4, 5, 6];
    for element in arr {
        println!("Element**2 = {}", element * element);
    }
    println!();

    for i in 1..10 {
        println!("{i}");
    }
    println!();

    for i in 0..10 {
        println!("fib({i}): {}", fib(i));
    }
    println!();

    for i in 0..10 {
        println!("fib_iter({i}): {}", fib_iter(i));
    }
    println!();
}

fn fib(n: i32) -> i32 {
    if n < 2 { return n }

    return fib(n - 1) + fib(n - 2)
}

fn fib_iter(n: i32) -> i32 {
    if n < 2 { return n }
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for _ in 2..n+1 {
        c = a + b;
        a = b;
        b = c;
    }

    return c
}