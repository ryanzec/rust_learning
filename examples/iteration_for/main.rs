fn main() {
    let values = vec![1, 2, 3, 4, 5];

    // this internally calls the iter method on the vector (does not transfer ownership of the vector)
    for value in &values {
        println!("Value: {}", value);
    }

    println!("Values: {:?}", values);

    // this internally calls the into_iter method on the vector (transfer ownership of the vector)
    for value in values.clone() {
        println!("Value: {}", value);
    }

    // the vec is not longer available due to the for loop moving ownership
    // println!("Values: {:?}", values);
}
