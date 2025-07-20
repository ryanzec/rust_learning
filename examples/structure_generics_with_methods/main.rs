#![allow(unused)]
#[derive(Debug)]
struct Chest<T> {
    name: String,
    contents: T,
}

#[derive(Debug)]
struct Bag;

// method defined here will only be available for Chests whose context is of type Bag
impl Chest<Bag> {
    fn open_bag(&self) {
        println!("Opening bag...");
    }
}

// for method that should be available for all chests, this is the format
impl<T> Chest<T> {
    fn open(&self) {
        println!("Opening chest...");
    }
}

fn main() {
    let chest = Chest {
        name: String::from("Treasure Chest"),
        contents: String::from("Gold"),
    };

    let chest2 = Chest {
        name: String::from("Treasure Chest"),
        contents: Bag,
    };

    // this does not work because chest is not of type Bag for its contens
    // chest.open_bag();
    chest2.open_bag();

    println!("{:?}", chest);
    println!("{:?}", chest2);
}
