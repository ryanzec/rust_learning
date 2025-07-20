fn main() {
    // start off as a string
    let size = "19";

    println!("size: {size}");

    // reassigning the same variable giving it a new type
    let size = size.parse::<i32>().unwrap();

    println!("size: {size}");
}
