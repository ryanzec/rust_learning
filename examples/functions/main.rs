// use a skinny arrow to define the return type of the function
fn add(number1: i32, number2: i32) -> i32 {
    return number1 + number2;
}

// while this alternative works too, in my opinion, it is not as clear as the first one and the typing saved is not
// worth it, there are other contexts where this is the only way to assign so it should be used in those cases only
fn add_alternative(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

fn main() {
    let results = add(2, 2);
    let results_alternative = add_alternative(2, 2);

    println!("2 + 2 = {results}");
    println!("2 + 2 = {results_alternative}");
}
