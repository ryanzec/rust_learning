use std::vec;

fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from("World");
    let string3 = String::from("!");

    // when using the variables to create a vector, ownership is transferred to the vector.
    let values = vec![string1, string2, string3];

    // would error because of ownership transfer
    // println!("{}, {}, {}", string1, string2, string3);
    println!("{:?}", values);

    let mut values_to_loop = values.clone();

    while let Some(value) = values_to_loop.pop() {
        println!("{}", value);
    }

    // you can not take ownership of a value from a vector but you can reference it
    // let value_reference = values[0];
    let value_reference = &values[0];

    println!("Reference to the first element: {}", value_reference);
    println!("{:?}", values);

    // get similar to arrays
    let value_get = values.get(0);
    println!("{:?}", value_get);
}
