#![allow(dead_code)]
#![allow(unused)]

use std::cmp::PartialEq;
use std::fmt::Debug;

trait Hashable {
    fn hash(&self) -> usize;
}

#[derive(Default, Clone)]
struct HashCell<Key, Value> {
    key: Key,
    value: Value,
    taken: bool,
}

struct HashBrown<Key, Value> {
    cells: Vec<HashCell<Key, Value>>,
    taken_count: usize,
}

impl<Key: Default + Clone + Hashable + Debug + PartialEq, Value: Default + Clone + Debug>
    HashBrown<Key, Value>
{
    fn new() -> Self {
        const INITIAL_CAPACITY: usize = 8;
        Self {
            cells: vec![HashCell::<_, _>::default(); INITIAL_CAPACITY],
            taken_count: 0,
        }
    }

    fn debug_dump(&self) {
        for cell in self.cells.iter() {
            if cell.taken {
                println!("{:?} -> {:?} -> {:?}", cell.key, cell.value, cell.taken);
            } else {
                println!("x");
            }
        }
    }

    // TODO:
    fn extend(&mut self) {
        assert!(self.cells.len() > 0);
        let mut new_self = Self {
            cells: vec![HashCell::<_, _>::default(); self.cells.len() * 2],
            taken_count: 0,
        };

        for cell in self.cells.iter() {
            if cell.taken {
                new_self.insert(cell.key.clone(), cell.value.clone());
            }
        }

        *self = new_self;
    }

    fn insert(&mut self, key: Key, value: Value) {
        if let Some(old_val) = self.get_mut(&key) {
            *old_val = value;
        } else {
            if self.taken_count >= self.cells.len() {
                self.extend();
            }
            assert!(self.taken_count < self.cells.len());

            let mut index = key.hash() % self.cells.len();
            while self.cells[index].taken {
                index = (index + 1) % self.cells.len();
            }

            self.cells[index].taken = true;
            self.cells[index].key = key;
            self.cells[index].value = value;
            self.taken_count += 1;
        }
    }

    fn get_index(&self, key: &Key) -> Option<usize> {
        let mut index = key.hash() % self.cells.len();
        for i in 0..self.cells.len() {
            if self.cells[index].taken == false {
                break;
            }
            if self.cells[index].key == *key {
                break;
            }
            index = (index + 1) % self.cells.len();
        }
        if self.cells[index].taken && self.cells[index].key == *key {
            return Some(index);
        } else {
            return None;
        }
    }

    fn get(&self, key: &Key) -> Option<&Value> {
        return self.get_index(key).map(|index| &self.cells[index].value);
    }

    fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        return self
            .get_index(key)
            .map(|index| &mut self.cells[index].value);
    }
}

impl Hashable for String {
    // http://www.cse.yorku.ca/~oz/hash.html
    fn hash(&self) -> usize {
        let mut result = 5381;
        for c in self.bytes() {
            result = ((result << 5) + result) + c as usize;
        }
        return result;
    }
}

fn main() {
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Hashable for i32 {
        fn hash(&self) -> usize {
            *self as usize
        }
    }

    #[test]
    fn test_new() {
        let brownies: HashBrown<i32, String> = HashBrown::new();
        assert_eq!(brownies.cells.len(), 8);
        assert_eq!(brownies.taken_count, 0);
    }

    #[test]
    fn test_insert_and_get() {
        let mut brownies = HashBrown::<i32, String>::new();
        brownies.insert(1, "one".to_string());
        brownies.insert(2, "two".to_string());

        assert_eq!(brownies.get(&1), Some(&"one".to_string()));
        assert_eq!(brownies.get(&2), Some(&"two".to_string()));
        assert_eq!(brownies.get(&3), None);
    }

    #[test]
    fn test_insert_update() {
        let mut brownies = HashBrown::<i32, String>::new();
        brownies.insert(1, "one".to_string());
        brownies.insert(1, "updated one".to_string());

        assert_eq!(brownies.get(&1), Some(&"updated one".to_string()));
    }

    #[test]
    fn test_get_mut() {
        let mut brownies = HashBrown::<i32, String>::new();
        brownies.insert(1, "one".to_string());

        if let Some(value) = brownies.get_mut(&1) {
            *value = "updated one".to_string();
        }

        assert_eq!(brownies.get(&1), Some(&"updated one".to_string()));
    }

    #[test]
    fn test_get_index() {
        let mut brownies = HashBrown::<i32, String>::new();
        brownies.insert(1, "one".to_string());

        assert!(brownies.get_index(&1).is_some());
        assert!(brownies.get_index(&2).is_none());
    }

    #[test]
    fn test_hash_collision() {
        let mut brownies = HashBrown::<i32, String>::new();
        brownies.insert(1, "one".to_string());
        brownies.insert(9, "nine".to_string()); // This should cause a collision with 1

        assert_eq!(brownies.get(&1), Some(&"one".to_string()));
        assert_eq!(brownies.get(&9), Some(&"nine".to_string()));
    }

    #[test]
    fn test_extend() {
        let mut brownies = HashBrown::<i32, String>::new();
        for i in 0..9 {
            brownies.insert(i, i.to_string());
        }

        // The HashBrown should have extended its capacity
        assert!(brownies.cells.len() > 8);

        // Check if all inserted elements are still accessible
        for i in 0..9 {
            assert_eq!(brownies.get(&i), Some(&i.to_string()));
        }
    }

    #[test]
    fn test_with_string_keys() {
        let mut brownies = HashBrown::<String, i32>::new();
        brownies.insert("one".to_string(), 1);
        brownies.insert("two".to_string(), 2);

        assert_eq!(brownies.get(&"one".to_string()), Some(&1));
        assert_eq!(brownies.get(&"two".to_string()), Some(&2));
        assert_eq!(brownies.get(&"three".to_string()), None);
    }
}
