fn main() {
    let values = vec![1, 2, 3, 4, 5];

    // iter creates an iterator of references to the values, the original vector still owns the data
    let values_iterator = values.iter();

    println!("{:?} values", values_iterator);

    // into_iter transfers ownership of the vector to the iterator
    let values_into_iterator = values.into_iter();

    println!("{:?} values", values_into_iterator);
}
