// if a function is returning a reference and it is passed in one argument, the lifetime of the return in implicitly
// that of the argument so the lifetime annotation is not needed but showing for clarity of the example
fn identity<'a>(x: &'a str) -> &'a str {
    x
}

// in order to return a reference from a function that is passed in multiple reference, you must provide a lifetime
// annotation to let the borrow checker know that which arguments the return could be tied to (and the borrow checker
// will make the the lifetime of the return value is valid for the smallest lifetime)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }

    y
}

fn main() {
    let value1 = String::from("Hello");
    let value2 = String::from("World");
    let longest = longest(&value1, &value2);

    println!("{}", identity(longest));
    println!("{}", longest);
}
