use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::Hash;

type OperationResult<'a, T> = Result<T, OperationError<'a>>;

#[derive(Debug)]
pub struct OperationError<'a> {
    message: &'a str,
}

impl<'a> Error for OperationError<'a> {
    fn description(&self) -> &str {
        self.message
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl<'a> fmt::Display for OperationError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

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
    pub fn insert<'a>(&mut self, l: T, r: U) -> OperationResult<'a, (T, U)> {
        if self.left_to_right.get(&l).is_some() && self.left_to_right.get(&l).unwrap() != &r {
            return Err(OperationError {
                message: "Collision with existing entry",
            });
        }
        self.left_to_right.insert(l, r);
        self.right_to_left.insert(r, l);
        Ok((l, r))
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
        let result = map.insert("abc", "xyz");

        assert!(result.is_ok());
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn insert_repeat() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");
        let result = map.insert("abc", "xyz");
        assert!(result.is_ok());
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn insert_other_value() {
        let mut map: BiMap<&str, &str> = BiMap::new();
        map.insert("abc", "xyz");
        let result = map.insert("abc", "123");

        assert!(result.is_err());
        assert_eq!(map.size(), 1);
    }
}
