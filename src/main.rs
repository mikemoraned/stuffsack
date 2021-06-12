use std::collections::HashMap;
use bloomfilter::Bloom;
use deepsize::{DeepSizeOf, Context};

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

struct BloomMap {
    bloom: Bloom<String>,
    direct: HashMap<String, bool>
}

impl BloomMap {
    fn get(&self, k: &String) -> Option<&bool> {
        if self.direct.contains_key(k) {
            self.direct.get(k)
        }
        else {
            if self.bloom.check(k) {
                Some(&true)
            }
            else {
                Some(&false)
            }
        }
    }
}

impl DeepSizeOf for BloomMap {
    fn deep_size_of_children(&self, context: &mut Context) -> usize
    {
        self.bloom.bitmap().deep_size_of_children(context)
            + self.direct.deep_size_of_children(context)
    }
}

fn compress(original: &HashMap<String, bool>) -> BloomMap {
    let bitmap_size = 1024 / 10;
    let mut bloom : Bloom<String> = Bloom::new(bitmap_size, original.len());
    for (key, value) in original {
        if *value {
            bloom.set(key);
        }
    }

    let mut direct: HashMap<String, bool> = HashMap::new();
    for (key, value) in original {
        if *value != bloom.check(key) {
            direct.insert(key.clone(), value.clone());
        }
    }

    BloomMap {
        bloom, direct
    }
}

fn main() {
    let key_length: usize = 30;
    let num_entries = 1000;

    let mut plain: HashMap<String, bool> = HashMap::new();
    (0..num_entries).for_each(|_| {
        plain.insert(random_key(key_length),random_value());
    });

    println!("{:?}", plain);

    let bloom_map = compress(&plain);

    let plain_size = plain.deep_size_of();
    let final_size = bloom_map.deep_size_of();
    let saving_percent = 100.0 * (1.0 - (final_size as f32 / plain_size as f32));

    println!("plain size: {}, final total: {}, saving: {}",
             plain_size,
             final_size,
             saving_percent);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{random_key, random_value, compress};

    #[test]
    fn same_output() {
        let key_length: usize = 30;
        let num_entries = 1000;

        let mut original: HashMap<String, bool> = HashMap::new();
        (0..num_entries).for_each(|_| {
            original.insert(random_key(key_length),random_value());
        });

        let bloom_map = compress(&original);

        for key in original.keys() {
            assert_eq!(bloom_map.get(key), original.get(key));
        }
    }
}
