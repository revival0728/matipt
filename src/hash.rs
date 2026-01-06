#![allow(dead_code)]

use std::{
    hash::{BuildHasher, Hasher, RandomState},
    ops::Index,
};

// HashMap that accepts both String and str as key
// As long as str or String contains same bytes, they have same key
pub struct SSMap<V> {
    hasher_builder: RandomState,
    table_len: u64,
    table: Vec<Vec<(String, V)>>,
}

impl<V> SSMap<V>
where
    V: Clone,
{
    pub fn new() -> SSMap<V> {
        let table = vec![Vec::new(); 1000];

        SSMap {
            hasher_builder: RandomState::new(),
            table_len: table.len() as u64,
            table,
        }
    }

    pub fn with_capacity(size: usize) -> SSMap<V> {
        let table = vec![Vec::new(); size];

        SSMap {
            hasher_builder: RandomState::new(),
            table_len: table.len() as u64,
            table,
        }
    }

    fn distribute(&self, u: u64) -> usize {
        let r = u64::MAX / self.table_len;
        (u / r) as usize
    }

    fn hash_str(&self, s: &str) -> usize {
        let mut hasher = self.hasher_builder.build_hasher();
        hasher.write(s.as_bytes());
        self.distribute(hasher.finish())
    }

    fn hash_string(&self, s: &String) -> usize {
        let mut hasher = self.hasher_builder.build_hasher();
        hasher.write(s.as_bytes());
        self.distribute(hasher.finish())
    }

    fn get_from_hashv<S>(&self, hv: usize, k: &S) -> Option<&V>
    where
        String: PartialEq<S>,
    {
        if self.table[hv].len() == 1 {
            return Some(&self.table[hv][0].1);
        }
        for (s, v) in &self.table[hv] {
            if s == k {
                return Some(v);
            }
        }
        None
    }

    pub fn get_from_str(&self, k: &str) -> Option<&V> {
        let hv = self.hash_str(k);
        self.get_from_hashv(hv, &k)
    }

    pub fn get_from_string(&self, k: &String) -> Option<&V> {
        let hv = self.hash_str(k);
        self.get_from_hashv(hv, k)
    }

    fn get_mut_from_hashv<S>(&mut self, hv: usize, k: &S) -> Option<&mut V>
    where
        String: PartialEq<S>,
    {
        if self.table[hv].len() == 1 {
            return Some(&mut self.table[hv][0].1);
        }
        for (s, v) in &mut self.table[hv] {
            if s == k {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut_from_str(&mut self, k: &str) -> Option<&mut V> {
        let hv = self.hash_str(k);
        self.get_mut_from_hashv(hv, &k)
    }

    pub fn get_mut_from_string(&mut self, k: &String) -> Option<&mut V> {
        let hv = self.hash_str(k);
        self.get_mut_from_hashv(hv, k)
    }

    pub fn insert_with_str(&mut self, k: &str, v: V) {
        let hv = self.hash_str(k);
        for (s, val) in &mut self.table[hv] {
            if s == k {
                *val = v.clone();
                return;
            }
        }
        self.table[hv].push((k.to_string(), v));
    }

    pub fn insert_with_string(&mut self, k: &String, v: V) {
        let hv = self.hash_string(k);
        for (s, val) in &mut self.table[hv] {
            if s == k {
                *val = v.clone();
                return;
            }
        }
        self.table[hv].push((k.to_string(), v));
    }

    pub fn contains_key_with_str(&self, k: &str) -> bool {
        self.get_from_str(k).is_some()
    }

    pub fn contains_key_with_string(&self, k: &String) -> bool {
        self.get_from_string(k).is_some()
    }
}

impl<V> Index<&str> for SSMap<V>
where
    V: Clone,
{
    type Output = V;

    fn index(&self, index: &str) -> &Self::Output {
        self.get_from_str(index).unwrap()
    }
}

impl<V> Index<&String> for SSMap<V>
where
    V: Clone,
{
    type Output = V;

    fn index(&self, index: &String) -> &Self::Output {
        self.get_from_str(index).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::SSMap;

    #[test]
    fn hash() {
        let map = SSMap::<i32>::new();
        let str_hash = map.hash_str("abc");
        let string_hash = map.hash_string(&"abc".to_string());

        assert_eq!(str_hash, string_hash);
    }

    #[test]
    fn insert() {
        let mut map = SSMap::<i32>::new();
        map.insert_with_str("abc", 1);
        map.insert_with_str("def", 1);
        map.insert_with_string(&"abc".to_string(), 2);

        let abc_hv = map.hash_str("abc");
        let def_hv = map.hash_str("def");
        assert_eq!(map.table[abc_hv].len(), 1);
        assert_eq!(map.table[abc_hv][0].1, 2);
        assert_eq!(map.table[def_hv].len(), 1);
        assert_eq!(map.table[def_hv][0].1, 1);
    }

    #[test]
    fn get_and_index() {
        let mut map = SSMap::<i32>::new();
        map.insert_with_str("abc", 1);

        let str_get = map.get_from_str("abc").unwrap();
        let index_get = map["abc"];
        let string_get = map.get_from_string(&"abc".to_string()).unwrap();
        assert_eq!(*str_get, 1);
        assert_eq!(index_get, 1);
        assert_eq!(*string_get, 1);
    }

    #[test]
    fn get_mut() {
        let mut map = SSMap::<i32>::new();
        map.insert_with_str("str", 1);
        map.insert_with_str("string", 1);

        *map.get_mut_from_str("str").unwrap() = 2;
        *map.get_mut_from_string(&"string".to_string()).unwrap() = 2;

        assert_eq!(map["str"], 2);
        assert_eq!(map["string"], 2);
    }
}
