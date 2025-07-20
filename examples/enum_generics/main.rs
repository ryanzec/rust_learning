#![allow(dead_code)]
// to be able to print out the enum values
#[derive(Debug)]
enum DamageType<T> {
    Physical(T),
    Fire(T),
    Lightning(T),
    Cold(T),
    Poison(T),
}

fn main() {
    let damage_type = DamageType::Physical(10);

    println!("{damage_type:?}");
}
