trait Descriptive {
    fn get_description(&self) -> String {
        String::from("This is a non descriptive thing.")
    }
}

struct Item;

struct Item2;

impl Descriptive for Item {}

impl Descriptive for Item2 {
    fn get_description(&self) -> String {
        String::from("This is an item2.")
    }
}

fn main() {
    let item = Item;
    let item2 = Item2;

    println!("{}", item.get_description());
    println!("{}", item2.get_description());
}
