#![feature(nll)]
use std::{collections::HashMap, hash::Hash};

pub struct MemoMap<'a, K, V, F>
where
    K: Ord + Hash + 'a,
    F: FnMut(&'a K) -> V,
{
    values: HashMap<&'a K, V>,
    func: F,
}

impl<'a, K, V, F> MemoMap<'a, K, V, F>
where
    K: Ord + Hash + 'a,
    F: FnMut(&'a K) -> V,
{
    pub fn new(func: F) -> Self {
        Self {
            values: HashMap::new(),
            func,
        }
    }

    pub fn get(&mut self, key: &'a K) -> &V {
        if !self.values.contains_key(key) {
            let value = (self.func)(key);
            self.values.insert(key, value);
        }

        self.values.get(key).unwrap()
    }

    pub fn try_get(&self, key: &'a K) -> Option<&V> {
        self.values.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut memo = MemoMap::new(|n| n * 2);
        assert!(memo.try_get(&2).is_none());
        assert_eq!(memo.get(&2), &4);
        assert_eq!(memo.try_get(&2), Some(&4));
    }
}
