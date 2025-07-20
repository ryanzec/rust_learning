fn main() {
    let mut value = String::from("Hello");

    // push can be used to add a single character to the end of a string
    value.push(',');

    println!("{}", value);

    let to_add = String::from("orld");

    // push_str can be used to add another &str
    value.push_str(" w");
    value.push_str(&to_add);

    println!("{}", value);

    let first_name = String::from("John");
    let last_name = String::from("Doe");
    let suffix = String::from("!");

    // using the + for adding string only works with &str
    // keep in mind the ownership of first_name is trasnfered to full_name
    let full_name = first_name + &last_name + &suffix;

    println!("{}", full_name);
    // this would error as the ownership of first_name has been moved to full_name
    // println!("{} {}", first_name, last_name);
}
