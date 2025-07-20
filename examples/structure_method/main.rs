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

impl Enemy {
    fn damage(&self) -> u32 {
        self.damage
    }

    fn take_damage(&mut self, amount: u32) {
        self.health -= amount;
    }
}

fn main() {
    let enemy = Enemy {
        name: String::from("Goblin"),
        health: 100,
        damage: 10,
        damage_type: DamageType::Physical,
    };

    let enemy_name = enemy.name;

    print!("enemy name {}", enemy_name);

    // since would error since the assignment above moves ownership of enemy.name to enemy_name
    // println!("Enemy name: {}", enemy.name);
}
