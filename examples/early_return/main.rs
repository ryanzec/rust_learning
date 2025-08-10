fn main() {
    result_early_return();
    option_early_return();
}

fn result_early_return() {
    let result_value: Result<&str, &str> = Ok("test");

    // if the result is an error, then it will return early
    let Ok(result) = result_value else {
        // additional error handling can be added here
        return;
    };

    println!("Result: {}", result);
}

fn option_early_return() {
    let option_value: Option<&str> = Some("test");

    // if the option is None, then it will return early
    let Some(option) = option_value else {
        // additional error handling can be added here
        return;
    };

    println!("Option: {}", option);
}
