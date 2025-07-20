// the benefit of this over a regular tuple is with the type, you have added safety that you are passing in the correct
// type of data and 2 tuples could have the same types but very different values and this jsut provides that extra check
struct Result(i32, bool);

fn main() {
    let result = Result(100, true);

    println!("{}, {}", result.0, result.1);
}
