fn print_value(value: i32) {
    println!("value: {value}");
}

fn print_value_string(value: String) {
    println!("value: {value}");
}

fn print_value_string_reference(value: &String) {
    println!("value: {value}");
}

fn main() {
    let value = 123;

    // scalar values implement Copy trait so no ownership is transferred
    print_value(value);

    // String does
    let value = String::from("test");

    // String does not implement Copy trait so ownership is transferred
    print_value_string(value);

    // would error as the ownership when passed to print_value_string was transferred
    // println!("{value}");
    //

    let value = String::from("test");

    // this is passed by reference so ownership is not transferred
    print_value_string_reference(&value);

    // since the above was passed by reference, value is still owned by the main function
    println!("{value}");
}
