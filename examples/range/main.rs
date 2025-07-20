fn main() {
    // by default the end value is excluded
    let range_end_excluded = 1..5;

    println!("{range_end_excluded:?}");

    for number in range_end_excluded {
        println!("end excluded: {}", number);
    }

    println!("");

    // use the = to include the end value
    let range_end_included = 3..=5;

    println!("{range_end_included:?}");

    for number in range_end_included {
        println!("end included: {}", number);
    }

    println!("");

    // works for strings
    let range_end_excluded = 'c'..'f';

    println!("{range_end_excluded:?}");

    for letter in range_end_excluded {
        println!("end excluded: {}", letter);
    }

    println!("");

    // works for characters
    let range_end_included = 'c'..'e';

    println!("{range_end_included:?}");

    for letter in range_end_included {
        println!("end included: {}", letter);
    }
}
