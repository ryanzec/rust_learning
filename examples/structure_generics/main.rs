#![allow(unused)]
#[derive(Debug)]
struct Chest<T> {
    name: String,
    contents: T,
}

#[derive(Debug)]
struct Bag;

fn main() {
    let chest = Chest {
        name: String::from("Treasure Chest"),
        contents: String::from("Gold"),
    };

    let chest2 = Chest {
        name: String::from("Treasure Chest"),
        contents: Bag,
    };

    println!("{:?}", chest);
    println!("{:?}", chest2);
}
