fn main() {
    let valid_value = get_option(10);
    let invalid_value = get_option(-5);

    process_option(&valid_value);
    process_option(&invalid_value);

    println!("{:?}, {:?}", valid_value, invalid_value);
}

fn process_option(option: &Option<i32>) {
    match option {
        Some(value) => println!("Processed value: {}", value),
        None => println!("No value to process"),
    }
}

fn get_option(value: i32) -> Option<i32> {
    if value > 0 { Some(value) } else { None }
}
