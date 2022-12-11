use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};


pub fn gen_token() -> String {
    let mut rng = thread_rng();

    (&mut rng).sample_iter(Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}