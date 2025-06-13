
fn main() {

    print_info();

    safe_increment();
    mutable_reference();
    immutable_reference();
    ownership();
    dangling();
}

fn safe_increment() {

    println!("\n —————————— Safe Increment —————————— \n");

    let x = 3;
    let y = increment(x);

    // Here, x and y can be printed in any order. The increment function here
    // has only operated on values, so x and y each have read and own (RO)
    // permissions over their values (neither has write permissions, since
    // neither is mutable).

    println!("{x}");
    println!("{y}");
    println!("{x}");
}

fn increment(n: i32) -> i32 {
    // This is a value being returned. Here, n is a parameter that holds
    // a copy of the argument passed in by the caller. n exists only within
    // the current stack frame. This value will disappear from the stack when
    // this function returns and be assigned to a variable in the caller's
    // stack frame.
    n + 1
}

fn mutable_reference() {

    println!("\n —————————— Mutable Reference —————————— \n");

    // x is mutable, so it has RWO permissions on its value.
    let mut x = 10;

    // Here, y is a mutable reference to x. This means that while y is live, x
    // loses *all* its permissions (read, write, own).
    //
    // The reason for this is that having two references to the same
    // place *and both having write access* is a recipe for undefined behavior.
    let y = &mut x;
    mutable_reference_increment(y);

    // If we uncomment it, this line will fail because y has taken ownership.
    // println!("{x}");

    // This is the last point y is used. After it, x regains its permissions.
    println!("{y}");

    // Now x has regained its permissions, so we can print it.
    println!("{x}");
}

fn mutable_reference_increment(n: &mut i32) {
    *n += 1
}

fn immutable_reference() {

    println!("\n —————————— Immutable Reference —————————— \n");

    // s is a pointer with RO permissions on a heap-allocated string.
    let s = String::from("Hello... ");

    // t is a reference to s. t has R permissions on s. t is mutable, so it can be reassigned.
    let mut t = &s;

    // It is fine to print these in either order. In the third line, we dereference t.
    println!("{s}");
    println!("{t}");
    println!("{}", *t);

    // It is *not* fine to drop s if the reference t is live, since t would
    // become a dangling reference.
    // drop(s);
    println!("{t}\n");

    // This time, s is mutable (RWO permissions on its string in the heap).
    let mut w = String::from("world!");

    // t is now a reference to a mutable variable. Scary!
    // t can read w (R permissions), but w loses WO permissions while t lives.
    t = &w;

    // w and t can both be printed (both still have R permissions).
    println!("{w}");
    println!("{t}");

    // We *cannot* modify w while t lives. Modifying w could cause a change to the underlying way
    // the string data is stored in the heap, which could cause t to become dangling,
    // so it is blocked by the rust compiler. w cannot be dropped either.

    // w.push_str(" - JSOD ");
    // drop(w);
    println!("{t}");

    // Once t is no longer live, w regains its full permissions.
    w.push_str(" - JSOD ");
    println!("\n{w}\n");

    print!("{s}");
    println!("{w}");
}

fn ownership() {

    println!("\n —————————— Ownership —————————— \n");

    // s is a pointer to a heap-allocated string. s has RO permissions.
    let s = String::from("Hello world!");
    println!("{s}");
    take_ownership(s);

    // s can no longer be referenced because it was moved.
    // println!("{s}");

    let s = String::from("Hello world, again!");
    println!("\n{s}");
    take_ownership(s.clone());

    // s can now be referenced, because we used a clone above.
    println!("{s}");
}

fn take_ownership(t: String) {
    // In this stack frame, the parameter t has taken ownership over the heap memory
    // that s once pointed to. s has lost its ownership.
    println!("taking ownership: {t}");
}

fn dangling() {

    println!("\n —————————— Dangling —————————— \n");

    let s = dangle();
    println!("{s}");
}

fn dangle() -> String {
    let s = String::from("no dangling here");

    // Trying to return &s would return a reference to a variable located in the current
    // stack frame. This variable will be dropped when the current function pops off the
    // stack, so the pointer will be meaningless and is blocked by rust.
    // &s

    // Instead, we can return s itself, which owns the heap-allocated memory.
    s
}

fn print_info() {

    println!("\n —————————— Info —————————— \n");

    let a = 'a';
    print_type_of(&a);
    println!("sizeof(char): {}\n", size_of::<char>());

    let a1 = "abc";
    print_type_of(&a1);
    println!("sizeof(&str): {}\n", size_of::<&char>());

    let s = String::from(a1);
    print_type_of(&s);
    println!("sizeof(&str): {}\n", size_of::<String>());


    let i: i32 = 5;
    print_type_of(&i);
    println!("sizeof(i32): {}\n", size_of::<i32>());

    let mut v: Vec<i32> = vec![5, 2, 3, 8, 6, 4];
    println!("v: {:?}", v);
    v.sort();
    println!("v: {:?}", v);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
