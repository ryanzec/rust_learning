fn main() {
    // this creates a &str which is a fixed length string that can be modified and is stored in the generated executable
    let value = "Hello, world!";

    println!("{}", value);

    // a String can change length and is stored on the heap
    let value = String::from("Hello, world!");

    println!("{}", value)
}
