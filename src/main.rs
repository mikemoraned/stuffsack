use std::collections::HashMap;

use deepsize::{Context, DeepSizeOf};

mod compressed_map;

fn random_key(key_length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();

    (0..key_length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn random_value() -> bool {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    rng.gen_bool(0.5)
}

fn main() {
    let key_length: usize = 30;
    let num_entries = 1000;

    let mut plain: HashMap<String, bool> = HashMap::new();
    (0..num_entries).for_each(|_| {
        plain.insert(random_key(key_length),random_value());
    });

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
