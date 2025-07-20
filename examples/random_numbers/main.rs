use rand::Rng;

fn random_number() -> u32 {
    let mut rng = rand::rng();

    rng.random_range(0..3_999_999_999_)
}

fn random_float() -> f32 {
    let mut rng = rand::rng();

    rng.random_range(0.0..1.0)
}

fn main() -> () {
    println!("{}", random_number());
    println!("{}", random_float());
}
