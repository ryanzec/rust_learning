fn main() {
    let pi = 3.14159;
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // uses the display traits for formating
    println!("pi: {pi}");

    // for decimal precision
    println!("pi: {pi:.3}");

    // uses the debug trait for formatting
    println!("Numbers: {numbers:?}");

    // uses the debug trait for formatting with pretty printing
    println!("Numbers: {numbers:#?}");
}
