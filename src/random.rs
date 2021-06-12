use rand::Rng;
use rand_pcg::Pcg64;
use rand::prelude::*;

pub fn random_key_value_pairs(key_length: usize, size: usize) -> Vec<(String, bool)> {
    let mut rng = Pcg64::seed_from_u64(2);
    (0..size).map(|_| {
        (random_key(key_length, &mut rng), random_value(&mut rng))
    }).collect()
}

fn random_key(key_length: usize, rng: &mut rand_pcg::Pcg64) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    (0..key_length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn random_value(rng: &mut rand_pcg::Pcg64) -> bool {
    rng.gen_bool(0.5)
}
