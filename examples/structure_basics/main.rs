#![allow(dead_code, unused_variables)]

#[derive(Debug)]
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

    println!("Name: {}", enemy.name);
    println!("Health: {}", enemy.health);
    println!("Damage: {}", enemy.damage);
    println!("Damage Type: {:?}", enemy.damage_type);

    println!("{:?}", enemy);
}
