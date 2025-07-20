#![allow(dead_code, unused_variables)]
fn main() {
    let value1 = "hello world";
    let value1_slice = &value1[0..5];
    let value1_slice = &value1[..5]; // equivlant to above

    println!("value1_slice: {}", value1_slice);

    let value2 = String::from("hello world");
    let value2_slice = &value2[6..=10];
    let value2_slice = &value1[6..]; // equivlant to above

    // outofbounds access produced a panic
    // let value2_slice = &value2[6..=11];

    println!("value2_slice: {}", value2_slice);

    // since strings are index by bytes and not characters and not all characters are the same size, you can not get a specific byte, you always have to specify a range
    let value1_slice = &value1[0..1];
    // let value1_slice = &value1[0]; // this errors
}
