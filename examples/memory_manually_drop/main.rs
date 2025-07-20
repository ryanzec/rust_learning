fn main() {
    let value1 = String::from("test");

    println!("value1: {value1}");

    // while drop() is called by rust when a scope ends, it can be called eplciitly
    drop(value1);

    // would error since the value was explicitly dropped
    // println!("value1: {value1}");
}
