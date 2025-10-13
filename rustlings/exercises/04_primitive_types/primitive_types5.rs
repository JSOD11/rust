fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.

    let (name, age) = cat;
    println!("{name} is {age} years old");
    println!("{} is {} years old", cat.0, cat.1);
}
