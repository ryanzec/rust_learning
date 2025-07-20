#![allow(dead_code)]
#[derive(Debug)]
struct MyStruct {
    field1: String,
    field2: i32,
}

// to be able to print out the enum values
#[derive(Debug)]
enum DamageType {
    Physical,
    Fire,
    Lightning,
    Cold,
    Poison,
}

impl DamageType {
    fn is_elemental(&self) -> bool {
        match self {
            DamageType::Physical => false,
            DamageType::Fire => true,
            DamageType::Lightning => true,
            DamageType::Cold => true,
            DamageType::Poison => false,
        }
    }
}

fn main() {
    let damage_type = DamageType::Physical;
    let damage_type2 = DamageType::Cold;

    println!(
        "{:?} is elemental: {}",
        damage_type,
        damage_type.is_elemental()
    );
    println!(
        "{:?} is elemental: {}",
        damage_type2,
        damage_type2.is_elemental()
    );
}
