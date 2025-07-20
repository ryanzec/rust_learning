struct Person {
    // this will not work because rust need to know the size of the struct at compile time and
    // since the struct is referencing itself, there is no way for rust to know that
    //
    // since a `Box<T>` is a reference to data on the heap and that reference does have a fixed
    // size, using a box is how you would allow a struct to reference itself
    //
    // parent: Option<Person>,
    parent: Option<Box<Person>>,
    name: String,
}

fn main() -> () {
    let parent = Box::new(Person {
        name: String::from("Parent"),
        parent: None,
    });
    let child = Person {
        name: String::from("Child"),
        parent: Some(parent),
    };
    let orphan = Person {
        name: String::from("Child"),
        parent: None,
    };

    let child_parent_name = match child.parent {
        Some(parent) => parent.name,
        _ => String::from("Orphan"),
    };

    let orphan_parent_name = match orphan.parent {
        Some(parent) => parent.name,
        _ => String::from("Orphan"),
    };

    println!("parent of {} is {}", child.name, child_parent_name);
    println!("parent of {} is {}", child.name, orphan_parent_name);
}
