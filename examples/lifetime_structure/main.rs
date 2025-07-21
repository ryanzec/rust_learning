#![allow(dead_code)]

// lifetime annotations are needed when a struct has a reference field so ensure the struct does not outlive the
// references it has
#[derive(Debug)]
struct Data<'a> {
    value: &'a str,
}

fn main() {
    let data = Data { value: "test" };

    {
        let value = String::from("Hello, World!");

        println!("{:?}", value);

        // since value is dropped after this inner scope, data would no longer valid since it is now hold a dangling
        // reference so this would error
        // data.value = value.as_str();
    }

    println!("{:?}", data);
}
