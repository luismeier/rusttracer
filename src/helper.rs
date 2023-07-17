use rand::prelude::*;

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

pub fn random_double(min: f64, max: f64) {
    return min + (max - min) * random_double();
}
