trait Character {
    fn move_to(&self);
}

struct Player;

impl Character for Player {
    fn move_to(&self) -> () {
        println!("Player is moving")
    }
}

fn main() -> () {
    // while this might be the initial way you would think to do this, it does not work. the
    // reason is because `Character` is a trait, the rust compiler does not know how much
    // memory `player` at compile time which it needs in order to place things on the stack.
    //
    // using `Box<T>` is the simplest way to tell rust that we want to store this data on the
    // heap and since rust does not need to know the size at compile time for storing data there,
    // this will work
    //
    // `Box<T>` should be used if you are only going to have one reference, you can use `Rc<T>`
    // or `Arc<T>` if multiple references are needed
    //
    // let player: dyn Character;
    //
    // player = Player;
    let player: Box<dyn Character>;

    player = Box::new(Player);

    player.move_to();
}
