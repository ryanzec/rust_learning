#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
enum DamageType {
    Physical,
    Magical,
}

#[derive(Debug)]
struct Enemy {
    name: String,
    health: u32,
    damage: u32,
    damage_type: DamageType,
}

fn main() {
    let enemy = Enemy {
        name: String::from("Goblin"),
        health: 100,
        damage: 10,
        damage_type: DamageType::Physical,
    };

    let enemy2 = Enemy {
        name: String::from("Blue Goblin"),

        // to avoid ownership transfer, we clone the damage_type
        damage_type: enemy.damage_type.clone(),

        // since the rest are scalar types, we can simply copy them
        // this will skip any fields that are already set
        ..enemy
    };

    println!("{:?}", enemy);
    println!("{:?}", enemy2);
}
