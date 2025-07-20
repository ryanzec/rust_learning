fn even_or_odd(num: i32) -> &'static str {
    return if num % 2 == 0 { "even" } else { "odd" };
}

fn main() {
    let results = even_or_odd(123);

    println!("123 is {results}");
}
