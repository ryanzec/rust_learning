fn main() {
    // rut has not null like type so instead if a value can be missing, we use Option enum
    let value: Option<i32> = None;

    println!("{:?}", value);
}
