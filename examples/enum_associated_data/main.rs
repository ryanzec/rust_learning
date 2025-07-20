#![allow(dead_code)]
#[derive(Debug)]
struct MyStruct {
    field1: String,
    field2: i32,
}

//to be able to print out the enum values
#[derive(Debug)]
enum DamageType {
    // you can have data associated to an instant of a enum
    Physical(String),

    // you can use a struct as an associated value
    Fire(MyStruct),

    // you can also inline a struct
    Lightning { field1: String, field2: i32 },

    Cold,
    Poison,
}

fn main() {
    let damage_type = DamageType::Physical(String::from("Blunt"));

    match &damage_type {
        // when matching on a enum with associated data, this is how you do it
        DamageType::Physical(damage) => println!("Physical damage: {}", damage),
        DamageType::Fire(fire) => println!("Fire damage: {:?}", fire),
        DamageType::Lightning { field1, field2 } => {
            println!("Lightning damage: {}, {}", field1, field2)
        }

        DamageType::Cold => println!("Cold damage"),
        DamageType::Poison => println!("Poison damage"),
    }

    println!("{damage_type:?}");
}
