use std::hash::{DefaultHasher, Hash, Hasher};

const HASH_MAP_INITIAL_CAPACITY: usize = 16;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    counter: usize,
    capacity: usize,
}

impl<K: Eq + Hash, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: (0..HASH_MAP_INITIAL_CAPACITY).map(|_| Vec::new()).collect(),
            counter: 0,
            capacity: HASH_MAP_INITIAL_CAPACITY,
        }
    }

    pub fn len(&self) -> usize {
        self.counter
    }

    pub fn is_empty(&self) -> bool {
        self.counter == 0
    }

    pub fn contains(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let index = self.bucket_index(&key);
        let bucket = &self.buckets[index];

        for (k, v) in bucket {
            if k == &key {
                return Some(v);
            }
        }

        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.should_resize() {
            self.resize();
        }

        let index = self.bucket_index(&key);
        let bucket = &mut self.buckets[index];

        for (k, v) in bucket.iter_mut() {
            if k == &key {
                let _ = std::mem::replace(v, value);
                return;
            }
        }

        bucket.push((key, value));
        self.counter += 1;
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let index = self.bucket_index(&key);
        let bucket = &mut self.buckets[index];

        if let Some(pos) = bucket.iter().position(|(k, _)| k == &key) {
            let (_, v) = bucket.remove(pos);
            self.counter -= 1;
            return Some(v);
        }

        None
    }

    pub fn clear(&mut self) {
        for bucket in self.buckets.iter_mut() {
            bucket.clear();
        }
        self.counter = 0;
    }

    fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    fn bucket_index_for(key: &K, capacity: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % capacity
    }

    fn should_resize(&self) -> bool {
        !self.buckets.is_empty() && self.counter * 4 >= self.buckets.len() * 3
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_buckets: Vec<Vec<(K, V)>> = (0..new_capacity).map(|_| Vec::new()).collect();

        for bucket in self.buckets.iter_mut() {
            for (k, v) in bucket.drain(..) {
                let index = Self::bucket_index_for(&k, new_capacity);
                new_buckets[index].push((k, v));
            }
        }

        self.buckets = new_buckets;
        self.capacity = new_capacity;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_returns_zero_for_new_hash_map() {
        let h = HashMap::<String, String>::new();
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn is_empty_returns_true_for_new_hash_map() {
        let h = HashMap::<String, String>::new();
        assert!(h.is_empty());
    }

    #[test]
    fn insert_increases_len_from_zero_to_one() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        assert_eq!(h.len(), 1);
        assert!(!h.is_empty());
    }

    #[test]
    fn insert_multiple_distinct_keys_increases_len_each_time() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        h.insert("b".to_string(), "2".to_string());
        h.insert("c".to_string(), "3".to_string());
        assert_eq!(h.len(), 3);
    }

    #[test]
    fn insert_same_key_twice_does_not_increase_len() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        h.insert("a".to_string(), "2".to_string());
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn insert_same_key_many_times_keeps_len_one() {
        let mut h = HashMap::<String, String>::new();
        for i in 0..50 {
            h.insert("a".to_string(), i.to_string());
        }
        assert_eq!(h.len(), 1);
        assert!(!h.is_empty());
    }

    #[test]
    fn insert_many_keys_len_matches_count() {
        let mut h = HashMap::<String, String>::new();
        for i in 0..100 {
            h.insert(format!("k{i}"), format!("v{i}"));
        }
        assert_eq!(h.len(), 100);
    }

    #[test]
    fn get_returns_none_for_empty_hash_map() {
        let h = HashMap::<String, String>::new();
        assert!(h.get("a".to_string()).is_none());
    }

    #[test]
    fn get_returns_some_after_insert() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        assert_eq!(h.get("a".to_string()), Some(&"1".to_string()));
    }

    #[test]
    fn get_returns_none_for_missing_key() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        assert!(h.get("b".to_string()).is_none());
    }

    #[test]
    fn get_returns_updated_value_for_same_key() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        h.insert("a".to_string(), "2".to_string());
        assert_eq!(h.get("a".to_string()), Some(&"2".to_string()));
    }

    #[test]
    fn get_works_for_many_keys() {
        let mut h = HashMap::<String, String>::new();
        for i in 0..100 {
            h.insert(format!("k{i}"), format!("v{i}"));
        }
        assert_eq!(h.get("k0".to_string()), Some(&"v0".to_string()));
        assert_eq!(h.get("k50".to_string()), Some(&"v50".to_string()));
        assert_eq!(h.get("k99".to_string()), Some(&"v99".to_string()));
        assert!(h.get("k100".to_string()).is_none());
    }

    #[test]
    fn remove_returns_none_for_empty_hash_map() {
        let mut h = HashMap::<String, String>::new();
        assert!(h.remove("a".to_string()).is_none());
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());
    }

    #[test]
    fn remove_returns_some_and_decreases_len() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());

        let removed = h.remove("a".to_string());
        assert_eq!(removed, Some("1".to_string()));
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());
    }

    #[test]
    fn remove_missing_key_returns_none_and_len_unchanged() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        h.insert("b".to_string(), "2".to_string());

        let removed = h.remove("c".to_string());
        assert!(removed.is_none());
        assert_eq!(h.len(), 2);
    }

    #[test]
    fn remove_then_get_returns_none() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());

        assert!(h.remove("a".to_string()).is_some());
        assert!(h.get("a".to_string()).is_none());
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn remove_only_removes_one_key() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        h.insert("b".to_string(), "2".to_string());
        h.insert("c".to_string(), "3".to_string());

        assert_eq!(h.remove("b".to_string()), Some("2".to_string()));
        assert_eq!(h.len(), 2);

        assert_eq!(h.get("a".to_string()), Some(&"1".to_string()));
        assert!(h.get("b".to_string()).is_none());
        assert_eq!(h.get("c".to_string()), Some(&"3".to_string()));
    }

    #[test]
    fn remove_same_key_twice_second_time_returns_none() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());

        assert_eq!(h.remove("a".to_string()), Some("1".to_string()));
        assert!(h.remove("a".to_string()).is_none());
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn remove_many_keys_len_matches_remaining() {
        let mut h = HashMap::<String, String>::new();

        for i in 0..50 {
            h.insert(format!("k{i}"), format!("v{i}"));
        }
        assert_eq!(h.len(), 50);

        for i in 0..20 {
            assert_eq!(h.remove(format!("k{i}")), Some(format!("v{i}")));
        }

        assert_eq!(h.len(), 30);
        for i in 0..20 {
            assert!(h.get(format!("k{i}")).is_none());
        }
        for i in 20..50 {
            assert_eq!(h.get(format!("k{i}")), Some(&format!("v{i}")));
        }
    }

    #[test]
    fn contains_returns_false_for_empty_hash_map() {
        let h = HashMap::<String, String>::new();
        assert!(!h.contains("a".to_string()));
    }

    #[test]
    fn contains_returns_true_after_insert() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        assert!(h.contains("a".to_string()));
    }

    #[test]
    fn contains_returns_false_for_missing_key() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        assert!(!h.contains("b".to_string()));
    }

    #[test]
    fn contains_still_true_after_value_update_same_key() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        h.insert("a".to_string(), "2".to_string());
        assert!(h.contains("a".to_string()));
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn clear_makes_hash_map_empty() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        h.insert("b".to_string(), "2".to_string());
        h.insert("c".to_string(), "3".to_string());

        h.clear();

        assert_eq!(h.len(), 0);
        assert!(h.is_empty());
        assert!(!h.contains("a".to_string()));
        assert!(!h.contains("b".to_string()));
        assert!(!h.contains("c".to_string()));
    }

    #[test]
    fn clear_on_empty_hash_map_keeps_it_empty() {
        let mut h = HashMap::<String, String>::new();
        h.clear();
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());
    }

    #[test]
    fn can_insert_after_clear() {
        let mut h = HashMap::<String, String>::new();
        h.insert("a".to_string(), "1".to_string());
        h.clear();

        h.insert("x".to_string(), "9".to_string());

        assert_eq!(h.len(), 1);
        assert!(h.contains("x".to_string()));
        assert_eq!(h.get("x".to_string()), Some(&"9".to_string()));
    }
}
