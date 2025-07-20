// since a unit is a tuple with no values, the unit like structure is the same define like this, while it is uncommon,
// this structure does allow for methods to be defined on it which can be useful in certain use cases
#[derive(Debug)]
struct Empty;

fn main() {
    let empty = Empty;

    dbg!(empty);
}
