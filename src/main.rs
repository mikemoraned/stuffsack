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

#[derive(DeepSizeOf)]
struct SizedPlain {
    plain: HashMap<String, bool>
}

struct SizedBloom {
    bloom: Bloom<String>
}

impl DeepSizeOf for SizedBloom {
    fn deep_size_of_children(&self, context: &mut Context) -> usize
    {
        self.bloom.bitmap().deep_size_of_children(context)
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

    let bitmap_size = 1024 / 10;
    let mut bloom : Bloom<String> = Bloom::new(bitmap_size, num_entries);
    for (key, value) in &plain {
        if *value {
            bloom.set(key);
        }
    }

    println!("{:?}", bloom);

    let correctness = plain
        .iter()
        .map(|(key, value)| {
            value == &bloom.check(key)
        })
        .fold(HashMap::new(), |mut acc, correct| {
            *acc.entry(correct).or_insert(0) += 1;
            acc
        });

    println!("{:?}", correctness);

    println!("plain size: {}, bloom size: {}",
             (SizedPlain { plain }).deep_size_of(),
             (SizedBloom { bloom }).deep_size_of());
}
