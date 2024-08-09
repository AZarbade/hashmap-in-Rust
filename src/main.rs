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
                println!("{:?} -> {:?}", cell.key, cell.value);
            } else {
                println!("x");
            }
        }
    }

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

impl Hashable for i32 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

fn main() {
    // use std::collections::HashMap;
    // let mut foo: HashMap<i32, i32> = HashMap::new();
    let mut foo: HashBrown<i32, i32> = HashBrown::new();
    for _ in 0..1_000_000 {
        let key = rand::random::<i32>();
        if let Some(value) = foo.get_mut(&key) {
            *value += 1;
        } else {
            foo.insert(key, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, Clone, Debug, PartialEq)]
    struct TestKey(String);

    impl Hashable for TestKey {
        fn hash(&self) -> usize {
            self.0.hash()
        }
    }

    #[test]
    fn test_new() {
        let hb: HashBrown<TestKey, i32> = HashBrown::new();
        assert_eq!(hb.cells.len(), 8);
        assert_eq!(hb.taken_count, 0);
    }

    #[test]
    fn test_insert_and_get() {
        let mut hb = HashBrown::new();
        hb.insert(TestKey("key1".to_string()), 42);
        hb.insert(TestKey("key2".to_string()), 24);

        assert_eq!(hb.get(&TestKey("key1".to_string())), Some(&42));
        assert_eq!(hb.get(&TestKey("key2".to_string())), Some(&24));
        assert_eq!(hb.get(&TestKey("key3".to_string())), None);
    }

    #[test]
    fn test_update_existing_key() {
        let mut hb = HashBrown::new();
        hb.insert(TestKey("key1".to_string()), 42);
        hb.insert(TestKey("key1".to_string()), 100);

        assert_eq!(hb.get(&TestKey("key1".to_string())), Some(&100));
    }

    #[test]
    fn test_extend() {
        let mut hb = HashBrown::new();
        for i in 0..10 {
            hb.insert(TestKey(format!("key{}", i)), i);
        }

        assert!(hb.cells.len() > 8);
        for i in 0..10 {
            assert_eq!(hb.get(&TestKey(format!("key{}", i))), Some(&i));
        }
    }

    #[test]
    fn test_get_mut() {
        let mut hb = HashBrown::new();
        hb.insert(TestKey("key1".to_string()), 42);

        if let Some(value) = hb.get_mut(&TestKey("key1".to_string())) {
            *value = 100;
        }

        assert_eq!(hb.get(&TestKey("key1".to_string())), Some(&100));
    }

    #[test]
    fn test_collision_handling() {
        #[derive(Default, Clone, Debug, PartialEq)]
        struct CollisionKey(usize);

        impl Hashable for CollisionKey {
            fn hash(&self) -> usize {
                self.0 % 8 // Force collisions in the initial 8 buckets
            }
        }

        let mut hb = HashBrown::new();
        for i in 0..20 {
            hb.insert(CollisionKey(i), i as i32);
        }

        for i in 0..20 {
            assert_eq!(hb.get(&CollisionKey(i)), Some(&(i as i32)));
        }
    }

    #[test]
    fn test_string_hash() {
        let s1 = "Hello".to_string();
        let s2 = "World".to_string();
        assert_ne!(s1.hash(), s2.hash());
    }

    #[test]
    fn test_i32_hash() {
        let n1: i32 = 42;
        let n2: i32 = -42;
        assert_ne!(n1.hash(), n2.hash());
    }
}
