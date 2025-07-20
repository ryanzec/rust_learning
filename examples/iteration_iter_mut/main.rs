fn main() {
    let mut values = vec![String::from("Hello"), String::from("World")];

    for value in values.iter_mut() {
        println!("Value: {}", value);

        value.push_str("!");
    }

    println!("Values: {:?}", values);

    let mut values = vec![10, 20, 30];

    for value in values.iter_mut() {
        println!("Value: {}", value);

        // depending on the type of the iterator values, you might have to explicitly reference the value to modify it
        *value -= 1;
    }

    println!("Values: {:?}", values);
}
