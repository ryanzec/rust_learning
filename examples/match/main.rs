fn match_basics() {
    let number = 123;
    let results = match number {
        value if value > 1 => "greater then two",
        0 => "zero",
        1 => "pne",
        _ => "some other number",
    };

    println!("{number} is {results}");
}

fn match_multiple_checks() {
    let number = 2;
    let results = match number {
        0 | 2 | 4 | 6 | 8 => "even",
        1 | 3 | 5 | 7 | 9 => "odd",
        _ => "some other number",
    };

    println!("{number} is {results}");
}

fn match_named_catch_all() {
    let number = 2;
    let results = match number {
        0 | 2 | 4 | 6 | 8 => String::from("even"),
        1 | 3 | 5 | 7 | 9 => String::from("odd"),
        other_value => format!("some other number {}", other_value),
    };

    println!("{number} is {results}");
}

fn main() {
    match_basics();
    match_multiple_checks();
    match_named_catch_all();
}
