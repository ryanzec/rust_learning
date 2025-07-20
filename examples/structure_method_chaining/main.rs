#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct Enemy {
    name: String,
    health: u32,
    damage: u32,
}

impl Enemy {
    // by returning an instance to itself, this allows the method chaining above
    fn attack(&mut self, target: &mut Enemy) -> &mut Self {
        target.health -= self.damage;

        return self;
    }

    fn heal(&mut self, amount: u32) -> &mut Self {
        self.health += amount;

        return self;
    }
}

fn main() {
    let mut enemy = Enemy {
        name: String::from("Goblin"),
        health: 100,
        damage: 10,
    };
    let mut enemy2 = Enemy {
        name: String::from("Orc"),
        health: 150,
        damage: 15,
    };

    enemy.attack(&mut enemy2).heal(5);

    println!("{:?}", enemy);
}
