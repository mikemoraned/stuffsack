use std::collections::HashMap;

use bloomfilter::Bloom;
use deepsize::{Context, DeepSizeOf};
use std::hash::Hash;

pub struct CompressedMap<V: Sized> {
    bloom_filters: HashMap<V, Bloom<String>>,
    direct: HashMap<String, V>
}

impl<V: Sized> CompressedMap<V> {
    pub fn contains_key(&self, k: &String) -> bool {
        if self.direct.contains_key(k) {
            return true;
        }
        else {
            for (_, bloom) in &self.bloom_filters {
                if bloom.check(k) {
                    return true;
                }
            }
            return false;
        }
    }

    pub fn get(&self, k: &String) -> Option<&V> {
        if self.direct.contains_key(k) {
            self.direct.get(k)
        }
        else {
            for (value, bloom) in &self.bloom_filters {
                if bloom.check(k) {
                    return Some(&value)
                }
            }
            None
        }
    }
}

impl<V: DeepSizeOf + Sized> DeepSizeOf for CompressedMap<V> {
    fn deep_size_of_children(&self, context: &mut Context) -> usize
    {
        self.bloom_filters.keys().map(|k| {
            k.deep_size_of_children(context)
        }).sum::<usize>()
        + self.bloom_filters.values().map(|b| {
                b.bitmap().deep_size_of_children(context)
            }).sum::<usize>()
        + self.direct.deep_size_of_children(context)
    }
}

pub fn compress<V: Sized + Eq + Hash + Clone>(original: &HashMap<String, V>) -> CompressedMap<V> {
    let bitmap_size = 1024 * 10;
    let mut bloom_filters : HashMap<V, Bloom<String>> = HashMap::new();
    for (key, value) in original {
        if bloom_filters.contains_key(value) {
            bloom_filters.get_mut(value).unwrap().set(key);
        }
        else {
            let mut bloom = Bloom::new(bitmap_size, original.len());
            bloom.set(key);
            bloom_filters.insert(value.clone(), bloom);
        }
    }

    let possibly_incorrect = CompressedMap {
        bloom_filters: bloom_filters.clone(), direct: HashMap::new()
    };

    let mut direct: HashMap<String, V> = HashMap::new();
    for (key, value) in original {
        if value != possibly_incorrect.get(key).unwrap() {
            direct.insert(key.clone(), value.clone());
        }
    }

    CompressedMap {
        bloom_filters, direct
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::compressed_map::{compress, CompressedMap};
    use crate::random::random_key_value_pairs;
    use std::fmt::Debug;
    use deepsize::DeepSizeOf;

    #[derive(Clone, PartialEq, Eq, Hash, Debug, DeepSizeOf)]
    enum Value {
        A, B, C, D
    }

    fn example_bool_map() -> HashMap<String, bool> {
        let key_length: usize = 30;
        let num_entries = 1000;
        let values = [true, false];

        random_key_value_pairs(key_length, num_entries, 1, &values).iter().cloned().collect()
    }

    fn example_enum_map() -> HashMap<String, Value> {
        let key_length: usize = 30;
        let num_entries = 1000;
        let values = [Value::A, Value::B, Value::C, Value::D];

        random_key_value_pairs(key_length, num_entries, 1, &values).iter().cloned().collect()
    }

    #[test]
    fn bool_map_same_output() {
        let original = example_bool_map();
        let compressed = compress(&original);

        assert_all_values_eq(original, compressed)
    }

    #[test]
    fn enum_map_same_output() {
        let original = example_enum_map();
        let compressed = compress(&original);

        assert_all_values_eq(original, compressed)
    }

    fn assert_all_values_eq<V:PartialEq + Debug>(original: HashMap<String, V>, compressed: CompressedMap<V>) {
        for key in original.keys() {
            assert_eq!(compressed.get(key), original.get(key));
        }
    }

    #[test]
    fn bool_map_smaller_size() {
        let original = example_bool_map();
        let compressed = compress(&original);

        assert_smaller_size(original, compressed);
    }

    #[test]
    fn enum_map_smaller_size() {
        let original = example_enum_map();
        let compressed = compress(&original);

        assert_smaller_size(original, compressed);
    }

    fn assert_smaller_size<V:PartialEq + Debug + DeepSizeOf>(original: HashMap<String, V>, compressed: CompressedMap<V>) {
        let original_size = original.deep_size_of();
        let compressed_size = compressed.deep_size_of();

        assert_ne!(original_size, compressed_size);
        assert!(compressed_size < original_size);
    }
}
