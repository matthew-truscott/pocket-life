use rand::Rng;

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
