fn ownership_transfer() {
    let value1 = String::from("test");

    println!("value1: {value1}");

    // when a value on the heap is assigned like this, the asignee (value2) takes ownership of the value
    let value2 = value1;

    // no longer works as ownership has moves
    // println!("value1: {value1}");
    println!("value2: {value2}");
}

fn reference() {
    let value = String::from("test");
    let value_reference = &value;

    println!("value: {value}, borrowed_value: {value_reference}");
}

fn dereference() {
    let value = String::from("test");
    let value_reference = &value;

    // when use the deference operator *, you can embed the value in the string
    println!("value: {value}, borrowed_value: {}", *value_reference);
    // this does the same as the above because rust has a display trait for reference to get the display value
    println!("value: {value}, borrowed_value: {value_reference}");
}

// to avoid ownership transfer of heap data, you can use the clone method (if the type implements the Clone trait)
fn clone() {
    let value1 = String::from("test");
    let value2 = value1.clone();

    println!("value1: {value1}, value2: {value2}");
}

fn main() {
    ownership_transfer();
    reference();
    dereference();
    clone();
}
