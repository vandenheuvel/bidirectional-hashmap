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
    pub fn get_key(&self, l: &T) -> Option<&U> {
        get(&self.left_to_right, l)
    }
    pub fn get_value(&self, r: &U) -> Option<&T> {
        get(&self.right_to_left, r)
    }
    pub fn insert_key(&mut self, l: T, r: U) {
        insert(&mut self.left_to_right, &mut self.right_to_left, l, r);
    }
    pub fn insert_value(&mut self, r: U, l: T) {
        insert(&mut self.right_to_left, &mut self.left_to_right, r, l);
    }
    pub fn update_key(&mut self, l: &T, r: U) -> Option<U> {
        update(&mut self.left_to_right, &mut self.right_to_left, l, r)
    }
    pub fn update_value(&mut self, r: &U, l: T) -> Option<T> {
        update(&mut self.right_to_left, &mut self.left_to_right, r, l)
    }
    pub fn remove(&mut self, l: &T) -> Option<U> {
        remove(&mut self.left_to_right, &mut self.right_to_left, l)
    }
    pub fn remove_value(&mut self, r: &U) -> Option<T> {
        remove(&mut self.right_to_left, &mut self.left_to_right, r)
    }
    pub fn len(&self) -> usize {
        self.left_to_right.len()
    }
}

fn get<'a, T: Copy + Eq + Hash, U: Copy + Eq + Hash>(map: &'a HashMap<T, U>, key: &T) -> Option<&'a U> {
    map.get(key)
}

fn insert<T: Copy + Eq + Hash, U: Copy + Eq + Hash>(mut map1: &mut HashMap<T, U>, mut map2: &mut HashMap<U, T>, v1: T, v2: U) {
    assert!((!map1.contains_key(&v1) && !map2.contains_key(&v2)) ||
        (map1.get(&v1).is_some() && map1.get(&v1).unwrap() == &v2));
    map1.insert(v1, v2);
    map2.insert(v2, v1);
}

fn update<T: Copy + Eq + Hash, U: Copy + Eq + Hash>(mut map1: &mut HashMap<T, U>, mut map2: &mut HashMap<U, T>, v1: &T, v2: U) -> Option<U> {
    let old_v2 = remove(map1, map2, v1);
    insert(map1, map2, *v1, v2);
    old_v2
}

fn remove<T: Copy + Eq + Hash, U: Copy + Eq + Hash>(mut map1: &mut HashMap<T, U>, mut map2: &mut HashMap<U, T>, key: &T) -> Option<U> {
    if let Some(value) = map1.get(key) {
        map2.remove(value);
    }
    map1.remove(key)
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
        map.insert_key("abc", "xyz");

        assert_eq!(map.get_key(&"abc"), Some(&"xyz"));
        assert_eq!(map.get_value(&"xyz"), Some(&"abc"));
        assert_eq!(map.len(), 1);

        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert_value("xyz", "abc");

        assert_eq!(map.get_key(&"abc"), Some(&"xyz"));
        assert_eq!(map.get_value(&"xyz"), Some(&"abc"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn insert_repeat() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert_key("abc", "xyz");
        map.insert_key("abc", "xyz");

        assert_eq!(map.get_key(&"abc"), Some(&"xyz"));
        assert_eq!(map.get_value(&"xyz"), Some(&"abc"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    #[should_panic]
    fn insert_other_value() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert_key("abc", "xyz");
        map.insert_key("abc", "123");
    }

    #[test]
    fn remove() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert_key("abc", "xyz");

        assert_eq!(map.remove(&"abc"), Some("xyz"));
        assert_eq!(map.get_key(&"abc"), None);
        assert_eq!(map.get_value(&"xyz"), None);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn remove_add() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert_key("abc", "xyz");
        map.remove(&"abc");
        map.insert_key("abc", "xyz");

        assert_eq!(map.get_key(&"abc"), Some(&"xyz"));
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

        assert_eq!(map.get_key(&"abc"), None);
        assert_eq!(map.get_value(&"xyz"), None);
    }

    #[test]
    fn update() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert_key("abc", "xyz");
        map.update_key(&"abc", "def");

        assert_eq!(map.get_key(&"abc"), Some(&"def"));
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

        map1.insert_key("abc", "xyz");
        map2.insert_key("abc", "xyz");

        assert_eq!(map1, map2);
    }

    #[test]
    fn clone() {
        let mut map1: BiMap<&str, &str> = BiMap::new();
        let map2 = map1.clone();

        assert_eq!(map1, map2);

        map1.insert_key("abc", "def");
        let map2 = map1.clone();

        assert_eq!(map1, map2);
    }
}
