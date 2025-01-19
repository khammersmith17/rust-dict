use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::iter::Iterator;

/// An impelementation of Python style dict
/// An ordered map that can be indexed

struct Dictionary<K, V> {
    len: usize,
    capacity: usize,
    keys: Vec<K>,
    key_map: HashMap<K, usize>,
    values: Vec<V>,
}

impl<K, V> Display for Dictionary<K, V>
where
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("{\n");
        for (key, val) in self.keys.iter().zip(&self.values) {
            output.push_str(&format!("{}: {}\n", key, val));
        }
        output.push_str("}");
        write!(f, "{}", output)
    }
}

impl<K: std::cmp::PartialEq + Hash + Eq + Clone, V: Clone> Dictionary<K, V> {
    pub fn new() -> Dictionary<K, V> {
        Dictionary {
            len: 0,
            capacity: 20,
            keys: Vec::with_capacity(20),
            key_map: HashMap::with_capacity(20),
            values: Vec::with_capacity(20),
        }
    }
    /// a new instances of a Dictionary with default capacity

    pub fn with_capacity(size: usize) -> Dictionary<K, V> {
        Dictionary {
            len: 0,
            capacity: size,
            keys: Vec::with_capacity(size),
            key_map: HashMap::with_capacity(size),
            values: Vec::with_capacity(size),
        }
    }

    pub fn update(&mut self, key: K, value: V) {
        // check to see if dict is at capacity
        if self.len.saturating_sub(1) == self.capacity {
            self.capacity += 10;
            self.values.reserve(10);
            self.key_map.reserve(10);
            self.keys.reserve(10);
        }
        self.keys.push(key.clone());
        // inserting current len
        // new len - 1 -> new index
        self.key_map.insert(key, self.len);
        self.len += 1;
        self.values.push(value);
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        // get index from map
        // remove index keys and values
        // adjust all indexs > than index
        match self.key_map.remove(&key) {
            Some(index) => {
                let value = self.values.remove(index);
                let _ = self.keys.remove(index);
                for (_, i) in self.key_map.iter_mut() {
                    if *i > index {
                        *i -= 1;
                    }
                }
                self.len -= 1;

                Some(value)
            }
            None => None,
        }
    }

    pub fn values(&self) -> &Vec<V> {
        &self.values
    }

    pub fn keys(&self) -> &Vec<K> {
        &self.keys
    }

    pub fn get(&self, key: K) -> Option<V> {
        match self.key_map.get(&key) {
            Some(i) => Some(self.values[*i].clone()),
            None => None,
        }
    }

    pub fn get_index(&self, i: usize) -> Option<V> {
        if i >= self.len {
            return None;
        }
        Some(self.values[i].clone())
    }

    pub fn get_or(&self, key: K, default: Option<V>) -> Option<V> {
        match self.key_map.get(&key) {
            Some(i) => Some(self.values[*i].clone()),
            None => {
                if default.is_some() {
                    default
                } else {
                    None
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn reserve(&mut self, size: usize) {
        self.capacity += size;
        self.values.reserve(size);
        self.key_map.reserve(size);
        self.keys.reserve(size);
    }

    pub fn sort_by_keys(&mut self) {}

    pub fn sort_by_values(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_default() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.update(1, "my_string".into());
        dict.update(2, "my_string2".into());
        assert_eq!(dict.len(), 2);
        assert_eq!(dict.capacity(), 20);
    }

    #[test]
    fn get() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.update(1, "my_string".into());
        dict.update(2, "my_string2".into());
        assert_eq!(dict.get(1).unwrap(), String::from("my_string"));
        assert_eq!(dict.get(0), None);
    }

    #[test]
    fn get_default() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.update(1, "my_string".into());
        dict.update(2, "my_string2".into());
        assert_eq!(dict.get_or(3, None), None);
    }

    #[test]
    fn remove() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.update(1, "my_string".into());
        dict.update(2, "my_string2".into());
        assert_eq!(dict.remove(1).unwrap(), String::from("my_string"));
        assert_eq!(dict.get(1), None);
        assert_eq!(dict.get(2).unwrap(), String::from("my_string2"));
    }

    #[test]
    fn reserve() {
        let mut dict = Dictionary::<i32, String>::new();
        assert_eq!(dict.capacity(), 20);
        dict.reserve(10);
        assert_eq!(dict.capacity(), 30);
    }

    #[test]
    fn set_capacity() {
        let dict = Dictionary::<i32, String>::with_capacity(30);
        assert_eq!(dict.capacity(), 30);
    }

    #[test]
    fn values() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.update(1, "my_string".into());
        dict.update(2, "my_string2".into());
        assert_eq!(
            dict.values().to_owned(),
            vec![String::from("my_string"), String::from("my_string2")],
        );
        assert_eq!(
            dict.values(),
            &vec![String::from("my_string"), String::from("my_string2")],
        );
    }

    #[test]
    fn keys() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.update(1, "my_string".into());
        dict.update(2, "my_string2".into());
        assert_eq!(dict.keys().to_owned(), vec![1, 2],);
        assert_eq!(dict.keys(), &vec![1, 2],);
    }

    #[test]
    fn get_index() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.update(1, "my_string".into());
        dict.update(2, "my_string2".into());
        assert_eq!(dict.get_index(0), Some(String::from("my_string")));
        assert_eq!(dict.get_index(1), Some(String::from("my_string2")));
    }
}
