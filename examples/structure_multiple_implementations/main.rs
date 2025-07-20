#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct Enemy {
    name: String,
    health: u32,
    damage: u32,
}

// multiple Implementations of a struct are merged into one
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

impl Enemy {
    fn attack(&self, target: &mut Enemy) {
        target.health -= self.damage;
    }
}

fn main() {
    let enemy = Enemy::new(String::from("Goblin"), 100, 10);

    println!("{:?}", enemy);
}
