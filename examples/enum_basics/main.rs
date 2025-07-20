#![allow(dead_code)]
// to be able to print out the enum values
#[derive(Debug)]
enum DamageType {
    Physical,
    Fire,
    Lightning,
    Cold,
    Poison,
}

fn main() {
    let damage_type = DamageType::Physical;

    println!("{damage_type:?}");
}
