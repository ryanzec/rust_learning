fn main() {
    let mut values: Vec<i32> = Vec::new();

    println!("initial values: {:?}", values);

    // addd to end
    values.push(12);
    values.push(13);
    values.push(15);
    values.push(16);

    println!("values after push: {:?}", values);

    // add anywhere
    values.insert(0, 11);
    values.insert(3, 14);
    values.insert(6, 17);

    println!("values after insert: {:?}", values);

    values.remove(0);
    values.remove(values.len() - 1);

    println!("values after remove: {:?}", values);

    let popped_value = values.pop();

    println!("popped value: {:?}", popped_value);
    println!("values after pop: {:?}", values);

    let mut append_values: Vec<i32> = vec![101, 102, 103];

    values.append(&mut append_values);

    println!("values after append: {:?}", values);

    values[0] = 1000;

    println!("values after modification: {:?}", values);
}
