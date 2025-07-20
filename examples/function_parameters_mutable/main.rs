fn append_to_string(value: &mut String) {
    value.push_str(" appended");
}

fn main() {
    let mut value = String::from("test");

    append_to_string(&mut value);

    println!("value: {value}");
}
