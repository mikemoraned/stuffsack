use std::collections::HashMap;

use stuffsack::random::{random_key_value_pairs, random_key};
use stuffsack::compressed_map;
use rand_pcg::Pcg64;
use rand::SeedableRng;

fn main() {
    let key_length: usize = 30;
    let num_entries = 1000;
    let values = [true, false];

    let plain: HashMap<String, bool>
        = random_key_value_pairs(key_length, num_entries, 1, &values).into_iter().collect();

    let compressed_map = compressed_map::compress(&plain);

    let num_dummy_keys = 10000000;
    let mut rng = Pcg64::seed_from_u64(2);
    for attempt in 0..num_dummy_keys {
        let dummy_key = random_key(key_length, &mut rng);
        if compressed_map.contains_key(&dummy_key) && !plain.contains_key(&dummy_key) {
            println!("{}: `{}` example key that is in compressed map but not original map", attempt, dummy_key);
            break;
        }
    }
}
