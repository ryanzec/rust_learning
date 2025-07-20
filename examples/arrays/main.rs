fn main() {
    // arrays kept size once created
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("numbers: {:?}", numbers);

    // looping through an array
    let colors = ["red", "green", "blue"];

    for color in colors {
        println!("color: {}", color);
    }
}
