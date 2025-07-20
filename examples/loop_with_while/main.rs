fn main() {
    let mut values = vec![1, 2, 3, 4, 5];

    while let Some(value) = values.pop() {
        println!("Value: {}", value);
    }
}
