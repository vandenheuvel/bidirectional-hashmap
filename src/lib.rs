use std::collections::HashMap;
use std::hash::Hash;

pub struct BiMap<T, U> {
    left_to_right: HashMap<T, U>,
    right_to_left: HashMap<U, T>,
}

impl<T: Copy + Eq + Hash, U: Copy + Eq + Hash> BiMap<T, U> {
    pub fn new() -> BiMap<T, U> {
        BiMap {
            left_to_right: HashMap::new(),
            right_to_left: HashMap::new(),
        }
    }
    pub fn insert(&mut self, l: T, r: U) {
        assert!(self.left_to_right.get(&l).is_none() || self.left_to_right.get(&l).unwrap() == &r);
        self.left_to_right.insert(l, r);
        self.right_to_left.insert(r, l);
    }
    pub fn size(&self) -> usize {
        self.left_to_right.len()
    }
}


#[cfg(test)]
mod tests {
    use super::BiMap;

    #[test]
    fn create() {
        let map: BiMap<i32, i32> = BiMap::new();

        assert_eq!(map.size(), 0);
    }

    #[test]
    fn insert_single() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");

        assert_eq!(map.size(), 1);
    }

    #[test]
    fn insert_repeat() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");
        map.insert("abc", "xyz");

        assert_eq!(map.size(), 1);
    }

    #[test]
    #[should_panic]
    fn insert_other_value() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");
        map.insert("abc", "123");
    }
}
