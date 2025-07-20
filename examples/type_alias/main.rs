// type alias provides context for the type
type Meters = i32;

fn type_alias() {
    let track_length: Meters = 10;

    println!("track lenght is {track_length}");
}

fn main() {
    type_alias();
}
