pub struct Health {
    health: u32,
}

impl Health {
    pub fn new(health: u32) -> Self {
        Health { health }
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.health -= amount;
    }

    pub fn heal(&mut self, amount: u32) {
        self.health += amount;
    }
}
