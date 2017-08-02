use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BiMap<T: Clone + Copy + Eq + Hash, U: Clone + Copy + Eq + Hash> {
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
    pub fn get(&self, l: &T) -> Option<&U> {
        self.left_to_right.get(l)
    }
    pub fn get_value(&self, r: &U) -> Option<&T> {
        self.right_to_left.get(r)
    }
    pub fn insert(&mut self, l: T, r: U) {
        assert!((!self.left_to_right.contains_key(&l) && !self.right_to_left.contains_key(&r)) ||
            (self.left_to_right.get(&l).is_some() && self.left_to_right.get(&l).unwrap() == &r));
        self.left_to_right.insert(l, r);
        self.right_to_left.insert(r, l);
    }
    pub fn insert_value(&mut self, r: U, l: T) {
        assert!(self.left_to_right.get(&l).is_none() || self.left_to_right.get(&l).unwrap() == &r);
        self.left_to_right.insert(l, r);
        self.right_to_left.insert(r, l);
    }
    pub fn update(&mut self, l: &T, r: U) -> Option<U> {
        let old_r = self.remove(l);
        self.insert(*l, r);
        old_r
    }
    pub fn update_value(&mut self, r: &U, l: T) -> Option<T> {
        let old_l = self.remove_value(r);
        self.insert_value(*r, l);
        old_l
    }
    pub fn remove(&mut self, l: &T) -> Option<U> {
        if let Some(value) = self.left_to_right.get(l) {
            self.right_to_left.remove(value);
        }
        self.left_to_right.remove(l)
    }
    pub fn remove_value(&mut self, r: &U) -> Option<T> {
        if let Some(value) = self.right_to_left.get(r) {
            self.left_to_right.remove(value);
        }
        self.right_to_left.remove(r)
    }
    pub fn len(&self) -> usize {
        self.left_to_right.len()
    }
}


#[cfg(test)]
mod tests {
    use super::BiMap;

    #[test]
    fn create() {
        let map: BiMap<i32, i32> = BiMap::new();

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn insert_single() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");

        assert_eq!(map.get(&"abc"), Some(&"xyz"));
        assert_eq!(map.get_value(&"xyz"), Some(&"abc"));
        assert_eq!(map.len(), 1);

        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert_value("xyz", "abc");

        assert_eq!(map.get(&"abc"), Some(&"xyz"));
        assert_eq!(map.get_value(&"xyz"), Some(&"abc"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn insert_repeat() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");
        map.insert("abc", "xyz");

        assert_eq!(map.get(&"abc"), Some(&"xyz"));
        assert_eq!(map.get_value(&"xyz"), Some(&"abc"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    #[should_panic]
    fn insert_other_value() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");
        map.insert("abc", "123");
    }

    #[test]
    fn remove() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");

        assert_eq!(map.remove(&"abc"), Some("xyz"));
        assert_eq!(map.get(&"abc"), None);
        assert_eq!(map.get_value(&"xyz"), None);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn remove_add() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");
        map.remove(&"abc");
        map.insert("abc", "xyz");

        assert_eq!(map.get(&"abc"), Some(&"xyz"));
        assert_eq!(map.get_value(&"xyz"), Some(&"abc"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn remove_empty() {
        let mut map: BiMap<&str, &str> = BiMap::new();

        assert_eq!(map.remove(&"abc"), None);
        assert_eq!(map.len(), 0);
        assert_eq!(map.remove_value(&"xyz"), None);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn get_empty() {
        let map: BiMap<&str, &str> = BiMap::new();

        assert_eq!(map.get(&"abc"), None);
        assert_eq!(map.get_value(&"xyz"), None);
    }

    #[test]
    fn update() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");
        map.update(&"abc", "def");

        assert_eq!(map.get(&"abc"), Some(&"def"));
        assert_eq!(map.get_value(&"xyz"), None);
        assert_eq!(map.get_value(&"def"), Some(&"abc"));
        assert_eq!(map.len(), 1);
        assert_eq!(map.left_to_right.len(), map.right_to_left.len());
    }

    #[test]
    fn eq() {
        let mut map1: BiMap<&str, &str> = BiMap::new();
        let mut map2: BiMap<&str, &str> = BiMap::new();

        assert_eq!(map1, map2);

        map1.insert("abc", "xyz");
        map2.insert("abc", "xyz");

        assert_eq!(map1, map2);
    }

    #[test]
    fn clone() {
        let mut map1: BiMap<&str, &str> = BiMap::new();
        let map2 = map1.clone();

        assert_eq!(map1, map2);

        map1.insert("abc", "def");
        let map2 = map1.clone();

        assert_eq!(map1, map2);
    }
}
