fn main() {
    let value: Option<i32> = None;

    println!("{:?}", value.unwrap_or_default());
    println!("{:?}", value.unwrap_or(-1));
}
