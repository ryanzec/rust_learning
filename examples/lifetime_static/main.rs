fn main() {
    // a static lifetime make the value live for the entire program, &str have an implicit lifetime of 'static but
    // showing here explicitly for the example
    let value: &'static str = "Hello";

    println!("{}", value);
}
