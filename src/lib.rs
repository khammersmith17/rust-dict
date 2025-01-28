use std::cmp::{PartialEq, PartialOrd};
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

impl<
        K: PartialOrd + PartialEq + Hash + Eq + Clone + Ord + Copy,
        V: Clone + Ord + PartialEq + PartialOrd + Eq,
    > Dictionary<K, V>
{
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

    /// a new instance of a Dictionary with a reserved capacity
    /// Allows for the need to not dynamically resize when the size is somewhat known ahead of time
    pub fn with_capacity(size: usize) -> Dictionary<K, V> {
        Dictionary {
            len: 0,
            capacity: size,
            keys: Vec::with_capacity(size),
            key_map: HashMap::with_capacity(size),
            values: Vec::with_capacity(size),
        }
    }

    /// Add a key value pair to the dictionary
    /// This will be pushed to the end of the dictionary
    /// This will be resized when the dictionary is at full capacity
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

    /// remove an element from the dictionary by key name
    /// This will be worst case an O(3n) operation
    /// if the key is in the dictionary, the value with be returned, otherwise None will be
    /// returned
    /// # Example
    //  ```
    // let mut dict = Dictionary::<i32, String>::new();
    // dict.update(1, "my_string".into());
    // dict.update(2, "my_string2".into());
    // assert_eq!(dict.remove(1).unwrap(), String::from("my_string"));
    // assert_eq!(dict.get(1), None);
    // assert_eq!(dict.get(2).unwrap(), String::from("my_string2"));
    //  ```
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

    /// get a reference to the colleciton of values in the dictionary
    pub fn values(&self) -> &Vec<V> {
        &self.values
    }

    /// get a reference to the collection of keys in the dictionary
    pub fn keys(&self) -> &Vec<K> {
        &self.keys
    }

    /// get value by key
    /// returns an Option<V>
    pub fn get(&self, key: K) -> Option<V> {
        // get by key
        match self.key_map.get(&key) {
            Some(i) => Some(self.values[*i].clone()),
            None => None,
        }
    }

    /// get a value by index
    /// This method takes advantage of the ordered nature of the data structure
    pub fn get_index(&self, i: usize) -> Option<V> {
        if i >= self.len {
            return None;
        }
        Some(self.values[i].clone())
    }

    /// get with a default
    /// parallel to dict.get(key, default) in python
    /// if no default is provided, None will be returned
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

    /// the number of key value pairs in the dictionary
    pub fn len(&self) -> usize {
        self.len
    }

    /// get the current capacity of the dictionary
    /// the number of items the dictionary can currently hold
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// reserve additional capacity in the dictionary
    /// useful when you know you will need more than what you currently have
    /// same approach as when more space is revered in a Vec
    pub fn reserve(&mut self, size: usize) {
        self.capacity += size;
        self.values.reserve(size);
        self.key_map.reserve(size);
        self.keys.reserve(size);
    }

    pub fn sort_by_keys(&mut self) {
        // use built in sort to sort keys
        // iter through the map and swap each value in value vec
        // recompute map with new indexs
        self.keys.sort();
        // swap indexes in values
        for (new_i, key) in self.keys[..self.len / 2].iter().enumerate() {
            let old_i = *self.key_map.get(&key).unwrap();
            let temp = self.values[new_i].to_owned();
            self.values[new_i] = self.values[old_i].to_owned();
            self.values[old_i] = temp;
        }
        self.recompute_map();
    }

    #[inline]
    fn recompute_map(&mut self) {
        for (i, key) in self.keys.iter().enumerate() {
            let index = self.key_map.get_mut(&key).unwrap();
            *index = i;
        }
    }

    pub fn sort_by_values(&mut self) {
        // start with bubble sort
        // when we swap, swap both
        //TODO:
        //figure out how we can do a double

        for i in 0..self.len {
            for j in 1..self.len {
                if self.values[i] > self.values[j] {
                    // swap both keys and values
                    let temp_val = self.values[j].to_owned();
                    let temp_key = self.keys[j].to_owned();
                    self.values[j] = self.values[i].to_owned();
                    self.keys[j] = self.keys[i];
                    self.values[i] = temp_val;
                    self.keys[j] = temp_key;
                }
            }
        }
    }
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

    #[test]
    fn test_sort_keys() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.update(3, "my_string".into());
        dict.update(1, "my_string2".into());
        dict.update(2, "my_string3".into());
        dict.update(5, "my_string5".into());
        dict.sort_by_keys();
        assert_eq!(
            dict.values(),
            &vec![
                String::from("my_string2"),
                String::from("my_string3"),
                String::from("my_string"),
                String::from("my_string5"),
            ],
        );
        assert_eq!(dict.keys(), &vec![1, 2, 3, 5]);
    }
}
