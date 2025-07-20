#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct Enemy {
    name: String,
    health: u32,
    damage: u32,
}

impl Enemy {
    // by not defining self in the parameter list, this is a basically a static method on Enemy (not on instances of
    // Enemy)
    fn new(name: String, health: u32, damage: u32) -> Self {
        Self {
            name,
            health,
            damage,
        }
    }
}

fn main() {
    let enemy = Enemy::new(String::from("Goblin"), 100, 10);

    println!("{:?}", enemy);
}
