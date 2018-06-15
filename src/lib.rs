#![recursion_limit="128"]
use std::{collections::HashMap, hash::Hash};

#[macro_use]
extern crate delegate;

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

    pub fn with_capacity(capacity: usize, func: F) -> Self {
        Self {
            values: HashMap::with_capacity(capacity),
            func,
        }
    }

    delegate! {
        target self.values {
            pub fn capacity(&self) -> usize;

            pub fn reserve(&mut self, additional: usize);

            pub fn shrink_to_fit(&mut self);

            pub fn values(&mut self) -> impl Iterator<Item = &V>;

            pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V>;

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn clear(&mut self);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'a K, &V)> {
        self.values.iter().map(|(k, v)| (*k, v))
    }

    pub fn keys<'b>(&'b self) -> impl Iterator<Item = &'a K> + 'b {
        self.values.keys().map(|k| *k)
    }

    pub fn contains_key(&mut self, key: &'a K) -> bool {
        self.values.contains_key(key)
    }

    pub fn get(&mut self, key: &'a K) -> &V {
        if !self.values.contains_key(key) {
            let value = (self.func)(key);
            self.values.insert(key, value);
        }

        self.values.get(key).unwrap()
    }

    pub fn get_mut(&mut self, key: &'a K) -> &V {
        if !self.values.contains_key(key) {
            let value = (self.func)(key);
            self.values.insert(key, value);
        }

        self.values.get_mut(key).unwrap()
    }

    pub fn try_get(&self, key: &'a K) -> Option<&V> {
        self.values.get(key)
    }

    pub fn try_get_mut(&mut self, key: &'a K) -> Option<&mut V> {
        self.values.get_mut(key)
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
