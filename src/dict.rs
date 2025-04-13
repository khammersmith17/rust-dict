use std::cmp::{PartialEq, PartialOrd};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::iter::{IntoIterator, Iterator};
use std::vec::IntoIter;

/// An impelementation of Python style dict
/// An ordered map that can be indexed
#[derive(Debug)]
pub struct Dictionary<K, V> {
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

impl<K, V> Clone for Dictionary<K, V>
where
    K: Copy + Clone,
    V: Copy + Clone,
{
    fn clone(&self) -> Self {
        Dictionary {
            len: self.len.clone(),
            capacity: self.capacity.clone(),
            keys: self.keys.clone(),
            key_map: self.key_map.clone(),
            values: self.values.clone(),
        }
    }
}

impl<
        K: PartialOrd + PartialEq + Hash + Eq + Clone + Ord + Copy,
        V: Clone + Ord + PartialEq + PartialOrd + Eq,
    > PartialEq for Dictionary<K, V>
{
    fn eq(&self, rhs: &Self) -> bool {
        if self.values != rhs.values {
            return false;
        }

        if self.keys != rhs.keys {
            return false;
        }

        if self.key_map != rhs.key_map {
            return false;
        }
        true
    }
}

impl<
        K: PartialOrd + PartialEq + Hash + Eq + Clone + Ord + Copy,
        V: Clone + Ord + PartialEq + PartialOrd + Eq,
    > Dictionary<K, V>
{
    /// A new instances of a Dictionary with default capacity.
    pub fn new() -> Dictionary<K, V> {
        Dictionary {
            len: 0,
            capacity: 0,
            keys: Vec::new(),
            key_map: HashMap::new(),
            values: Vec::new(),
        }
    }

    /// A new instance of a Dictionary with a reserved capacity.
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

    /// Add a key value pair to the dictionary.
    /// This will be pushed to the end of the dictionary.
    /// This will be resized when the dictionary is at full capacity.
    pub fn push_back(&mut self, key: K, value: V) {
        // check to see if dict is at capacity
        if self.len == self.capacity {
            self.update_capacity();
        }
        self.keys.push(key.clone());
        // inserting current len
        // new len - 1 -> new index
        self.key_map.insert(key, self.len);
        self.len += 1;
        self.values.push(value);
    }

    fn update_capacity(&mut self) {
        let mut temp = self.capacity;
        let mut n = 0;
        while temp > 1 {
            temp = temp >> 1;
            n += 1
        }
        let new_capacity = 2 << n;
        let additional = new_capacity - self.capacity;
        self.values.reserve(additional);
        self.keys.reserve(additional);
        self.key_map.reserve(additional);
        self.capacity = new_capacity;
    }

    /// remove an element from the dictionary by key name
    /// This will be worst case an O(3n) operation
    /// if the key is in the dictionary, the value with be returned, otherwise None will be
    /// returned
    /// # Example
    /// ```
    /// use rust_dict::dict::Dictionary;
    ///
    /// let mut dict = Dictionary::<i32, String>::new();
    /// dict.push_back(1, "my_string".into());
    /// dict.push_back(2, "my_string2".into());
    /// assert_eq!(dict.remove(1).unwrap(), String::from("my_string"));
    /// assert_eq!(dict.get(1), None);
    /// assert_eq!(dict.get(2).unwrap(), String::from("my_string2"));
    /// ```
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

    /// Insert values to a particular index
    pub fn insert(&mut self, key: K, value: V, index: usize) {
        // insert key and value at i
        // then push_back the index map
        // increment all > i
        self.values.insert(index, value);
        self.keys.insert(index, key);

        for key in &self.keys[index + 1..] {
            let i = self.key_map.get_mut(&key).unwrap();
            *i += 1;
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
    /// returns an `Option<V>`
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
    pub fn get_or(&self, key: K, default: V) -> V {
        match self.key_map.get(&key) {
            Some(i) => self.values[*i].clone(),
            None => default,
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
        // old index -> new index
        // once we reach mid point, all are correct
        for (new_i, key) in self.keys[..self.len / 2].iter().enumerate() {
            let old_i = *self.key_map.get(&key).unwrap();
            let temp = self.values[new_i].to_owned();
            self.values[new_i] = self.values[old_i].to_owned();
            self.values[old_i] = temp;
        }
        // recompute the key value index map
        self.recompute_map();
    }

    #[inline]
    fn recompute_map(&mut self) {
        for (i, key) in self.keys.iter().enumerate() {
            let index = self.key_map.get_mut(&key).unwrap();
            *index = i;
        }
    }

    /// Sort the dictionary by values.
    /// keys
    /// # Example
    /// ```
    /// use rust_dict::dict::Dictionary;
    /// let mut dict = Dictionary::<i32, i32>::new();
    /// dict.push_back(3, 4);
    /// dict.push_back(1, 7);
    /// dict.push_back(2, 1);
    /// dict.push_back(5, 9);
    /// assert_eq!(dict.len(), 4);
    /// dict.sort_by_values();
    /// assert_eq!(dict.values(), &vec![1, 4, 7, 9],);
    /// assert_eq!(dict.keys(), &vec![2, 3, 1, 5]);
    /// ```
    pub fn sort_by_values(&mut self) {
        // start with bubble sort
        // when we swap, swap both
        // starting with bubble sort so we can swap both the keys and the values when sorting
        // there is probably a better way to do this
        for i in 0..self.len {
            let mut swapped = false;
            for j in 0..self.len - i - 1 {
                if self.values[j] > self.values[j + 1] {
                    swapped = true;
                    // swap both keys and values
                    let temp_val = self.values[j].to_owned();
                    let temp_key = self.keys[j].to_owned();
                    self.values[j] = self.values[j + 1].to_owned();
                    self.keys[j] = self.keys[j + 1].to_owned();
                    self.values[j + 1] = temp_val;
                    self.keys[j + 1] = temp_key;
                }
            }
            if !swapped {
                break;
            }
        }
        // recompute the key value index map
        self.recompute_map();
    }

    pub fn iter(self) -> DictIter<K, V> {
        DictIter {
            key_iter: self.keys.into_iter(),
            val_iter: self.values.into_iter(),
        }
    }
}

impl<K, V> Into<DictIter<K, V>> for Dictionary<K, V> {
    fn into(self) -> DictIter<K, V> {
        DictIter {
            key_iter: self.keys.into_iter(),
            val_iter: self.values.into_iter(),
        }
    }
}

pub struct DictIter<K, V> {
    key_iter: IntoIter<K>,
    val_iter: IntoIter<V>,
}

// Gets collect for free here
// collect will return a Vec<(K,V)>
impl<'a, K, V> Iterator for DictIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        let next_key = self.key_iter.next();
        let next_val = self.val_iter.next();
        debug_assert!(
            (next_key.is_some() && next_val.is_some())
                || (next_key.is_none() && next_val.is_none())
        );
        if next_key.is_some() {
            let Some(k) = next_key else {
                return None;
            };
            let Some(v) = next_val else { return None };
            Some((k, v))
        } else {
            None
        }
    }
}

impl<
        K: PartialOrd + PartialEq + Hash + Eq + Clone + Ord + Copy,
        V: Clone + Ord + PartialEq + PartialOrd + Eq,
    > Into<Dictionary<K, V>> for DictIter<K, V>
{
    fn into(self) -> Dictionary<K, V> {
        // utility to go back to the Dictionary
        debug_assert_eq!(self.key_iter.len(), self.val_iter.len());
        let len = self.key_iter.len();
        let capacity = (len as f32 * 1.1_f32) as usize;
        let mut keys: Vec<K> = Vec::with_capacity(capacity);
        let mut values: Vec<V> = Vec::with_capacity(capacity);
        let mut key_map: HashMap<K, usize> = HashMap::with_capacity(capacity);

        // iter through self and collect the the items to reconstruct the Dictionary
        for (i, (key, value)) in self.enumerate() {
            keys.push(key);
            values.push(value);
            key_map.insert(key, i);
        }
        Dictionary {
            len,
            capacity,
            keys,
            key_map,
            values,
        }
    }
}

impl<K, V> IntoIterator for Dictionary<K, V> {
    type Item = (K, V);
    type IntoIter = DictIter<K, V>;
    fn into_iter(self) -> DictIter<K, V> {
        DictIter {
            key_iter: self.keys.into_iter(),
            val_iter: self.values.into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictiter_to_dictionary() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.push_back(1, "my_string".into());
        dict.push_back(2, "my_string2".into());

        let mut dict2 = Dictionary::<i32, String>::new();
        dict2.push_back(1, "my_string".into());
        dict2.push_back(2, "my_string2".into());

        let dict2iter = dict2.into_iter();

        let dict2: Dictionary<i32, String> = dict2iter.into();
        assert_eq!(dict, dict2);
    }

    #[test]
    fn test_iter() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.push_back(1, "my_string".into());
        dict.push_back(2, "my_string2".into());

        let mut dict_iter = dict.into_iter();
        assert_eq!(dict_iter.next(), Some((1, "my_string".to_string())));
        assert_eq!(dict_iter.next(), Some((2, "my_string2".to_string())));
    }

    #[test]
    fn new_default() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.push_back(1, "my_string".into());
        dict.push_back(2, "my_string2".into());
        assert_eq!(dict.len(), 2);
        assert_eq!(dict.capacity(), 2);
    }

    #[test]
    fn get() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.push_back(1, "my_string".into());
        dict.push_back(2, "my_string2".into());
        assert_eq!(dict.get(1).unwrap(), String::from("my_string"));
        assert_eq!(dict.get(0), None);
    }

    #[test]
    fn get_default() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.push_back(1, "my_string".into());
        dict.push_back(2, "my_string2".into());
        assert_eq!(
            dict.get_or(3, String::from("my_string3")),
            String::from("my_string3")
        );
    }

    #[test]
    fn remove() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.push_back(1, "my_string".into());
        dict.push_back(2, "my_string2".into());
        assert_eq!(dict.remove(1).unwrap(), String::from("my_string"));
        assert_eq!(dict.get(1), None);
        assert_eq!(dict.get(2).unwrap(), String::from("my_string2"));
    }

    #[test]
    fn reserve() {
        let mut dict = Dictionary::<i32, String>::new();
        assert_eq!(dict.capacity(), 0);
        dict.reserve(10);
        assert_eq!(dict.capacity(), 10);
    }

    #[test]
    fn set_capacity() {
        let dict = Dictionary::<i32, String>::with_capacity(30);
        assert_eq!(dict.capacity(), 30);
    }

    #[test]
    fn values() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.push_back(1, "my_string".into());
        dict.push_back(2, "my_string2".into());
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
        dict.push_back(1, "my_string".into());
        dict.push_back(2, "my_string2".into());
        assert_eq!(dict.keys().to_owned(), vec![1, 2],);
        assert_eq!(dict.keys(), &vec![1, 2],);
    }

    #[test]
    fn get_index() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.push_back(1, "my_string".into());
        dict.push_back(2, "my_string2".into());
        assert_eq!(dict.get_index(0), Some(String::from("my_string")));
        assert_eq!(dict.get_index(1), Some(String::from("my_string2")));
    }

    #[test]
    fn test_sort_keys() {
        let mut dict = Dictionary::<i32, String>::new();
        dict.push_back(3, "my_string".into());
        dict.push_back(1, "my_string2".into());
        dict.push_back(2, "my_string3".into());
        dict.push_back(5, "my_string5".into());
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

    #[test]
    fn test_sort_values() {
        let mut dict = Dictionary::<i32, i32>::new();
        dict.push_back(3, 4);
        dict.push_back(1, 7);
        dict.push_back(2, 1);
        dict.push_back(5, 9);
        assert_eq!(dict.len(), 4);
        dict.sort_by_values();
        assert_eq!(dict.values(), &vec![1, 4, 7, 9],);
        assert_eq!(dict.keys(), &vec![2, 3, 1, 5]);
    }

    #[test]
    fn insert() {
        let mut dict = Dictionary::<i32, i32>::new();
        dict.push_back(3, 4);
        dict.push_back(1, 7);
        dict.push_back(2, 1);
        dict.push_back(5, 9);
        dict.insert(6, 7, 2);
        assert_eq!(dict.keys(), &vec![3, 1, 6, 2, 5]);
    }

    #[test]
    fn test_capacity_update() {
        let mut dict = Dictionary::<i32, i32>::new();
        assert_eq!(dict.capacity(), 0);
        dict.push_back(3, 4);
        assert_eq!(dict.capacity(), 2);
        dict.push_back(1, 7);
        dict.push_back(2, 1);
        assert_eq!(dict.capacity(), 4);
        dict.push_back(5, 9);
        dict.push_back(6, 10);
        assert_eq!(dict.capacity(), 8);
    }
}
