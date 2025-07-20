fn main() {
    println!("Hello, world! {}", identify::<i8>(42));
}

fn identify<TType>(value: TType) -> TType {
    value
}
