type MyTuple = (bool, i32);

fn main() {
    let my_tuple: MyTuple = (true, 42);

    println!("Tuple: {my_tuple:?}");

    // you are access tuple data with the numeric key but you can't inline when formatting
    println!("Tuple: ({}, {})", my_tuple.0, my_tuple.1);

    // you can also destructure tuple values into variables
    let (is_true, number) = my_tuple;

    println!("Is true: {is_true}, Number: {number}");
}
