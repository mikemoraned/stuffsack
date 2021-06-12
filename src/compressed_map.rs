use std::collections::HashMap;

use bloomfilter::Bloom;
use deepsize::{Context, DeepSizeOf};

pub struct CompressedMap {
    bloom: Bloom<String>,
    direct: HashMap<String, bool>
}

impl CompressedMap {
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

impl DeepSizeOf for CompressedMap {
    fn deep_size_of_children(&self, context: &mut Context) -> usize
    {
        self.bloom.bitmap().deep_size_of_children(context)
            + self.direct.deep_size_of_children(context)
    }
}

pub fn compress(original: &HashMap<String, bool>) -> CompressedMap {
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

    CompressedMap {
        bloom, direct
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::compressed_map::compress;
    use crate::random::{random_key, random_value};

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
