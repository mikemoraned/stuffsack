use std::collections::HashMap;

use deepsize::DeepSizeOf;
use crate::random::random_key_value_pairs;

mod compressed_map;
mod random;

fn main() {
    let key_length: usize = 30;
    let num_entries = 1000;
    let values = [true, false];

    let plain: HashMap<String, bool>
        = random_key_value_pairs(key_length, num_entries, 1, &values).into_iter().collect();

    println!("{:?}", plain);

    let compressed_map = compressed_map::compress(&plain);

    let plain_size = plain.deep_size_of();
    let final_size = compressed_map.deep_size_of();
    let saving_percent = 100.0 * (1.0 - (final_size as f32 / plain_size as f32));

    println!("plain size: {}, final total: {}, saving: {}",
             plain_size,
             final_size,
             saving_percent);
}
