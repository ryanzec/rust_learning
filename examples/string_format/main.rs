fn main() {
    let first_name = String::from("John");
    let last_name = String::from("Doe");
    let suffix = String::from("!");

    // there is no ownership transfer when using format!
    let full_name = format!("{} {}{}", first_name, last_name, suffix);

    println!("full name: {}", full_name);
    println!("first name: {}", first_name);
    println!("last name: {}", last_name);
    println!("suffix: {}", suffix);
}
