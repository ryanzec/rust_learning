fn main() {
    let damage_types = ["Physical", "Magical", "Healing"];
    let valid_value = damage_types.get(2);
    let invalid_value = damage_types.get(3);

    println!("{:?}, {:?}", valid_value, invalid_value);
}
