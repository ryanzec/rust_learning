fn main() {
    let mut count = 0;

    loop {
        count += 1;

        if count > 10 {
            // a loop with run indefinitely until a break is called
            break;
        }

        if count % 2 == 0 {
            println!("Even number: {count}");

            continue;
        }

        println!("Odd number: {count}");
    }
}
