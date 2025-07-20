fn main() {
    // this creates a &str which is a fixed length string
    let value = "Hello, world!";
    // string literal
    println!("{}", value);

    // escaping
    println!("C:\\windows\\path");
    println!(r"C:\windows\path"); // r in front of string is a raw string, no escaping

    // this creates a dynamic string
    #[allow(unused_variables)]
    let value = String::new();

    // this creates a dynamic string with a value
    let mut value = String::from("test");

    // for dynamic strings you can append using the push_str method
    value.push_str("string");
}
