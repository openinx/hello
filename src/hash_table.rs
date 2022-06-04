pub trait Hash: Eq {
    fn hash(&self) -> i64;
}

pub struct HashTable<K, V> {
    size: usize,
    buckets: Vec<Vec<Tuple<K, V>>>,
}

struct Tuple<K, V> {
    k: K,
    v: V,
}

fn index<K: Hash>(bucket_num: usize, key: &K) -> usize {
    let hash_code = key.hash();
    let num_i64 = bucket_num as i64;
    ((hash_code % num_i64 + num_i64) % num_i64) as usize
}

fn init_buckets<K, V>(bucket_num: usize) -> Vec<Vec<Tuple<K, V>>> {
    let mut buckets: Vec<Vec<Tuple<K, V>>> = Vec::with_capacity(bucket_num);
    for _ in 0..bucket_num {
        buckets.push(Vec::new());
    }
    buckets
}

impl<K, V> HashTable<K, V>
where
    K: Hash,
    V: Eq,
{
    pub fn new() -> HashTable<K, V> {
        HashTable {
            size: 0,
            buckets: init_buckets(4),
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let bucket_id = index(self.buckets.capacity(), key);
        let bucket = &mut self.buckets[bucket_id];

        for t in bucket {
            if t.k == *key {
                return Some(&mut t.v);
            }
        }

        None
    }

    fn extend(&mut self) {
        let mut new_buckets: Vec<Vec<Tuple<K, V>>> = init_buckets(self.buckets.capacity() * 2);

        for bucket in &mut self.buckets {
            while bucket.len() > 0 {
                let tuple = bucket.pop().unwrap();

                let new_bucket_id = index(new_buckets.capacity(), &tuple.k);
                let new_bucket = &mut new_buckets[new_bucket_id];
                new_bucket.push(tuple);
            }
        }

        self.buckets = new_buckets;
    }

    pub fn put(&mut self, key: K, val: V) {
        let bucket_id = index(self.buckets.capacity(), &key);

        match self.get_mut(&key) {
            Some(old_val) => *old_val = val,
            None => {
                let bucket = &mut self.buckets[bucket_id];
                bucket.push(Tuple { k: key, v: val });

                // Increment the size of hash table.
                self.size += 1;

                // Extend the buckets if possible.
                if self.size >= self.buckets.len() * 2 {
                    self.extend();
                }
            }
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let bucket_id = index(self.buckets.capacity(), &key);
        let bucket = &self.buckets[bucket_id];

        for t in bucket {
            if t.k == key {
                return Some(&t.v);
            }
        }
        None
    }

    pub fn delete(&mut self, key: K) -> Option<V> {
        let bucket_id = index(self.buckets.capacity(), &key);
        let bucket = &mut self.buckets[bucket_id];

        let mut key_pos: Option<usize> = None;
        for (pos, tuple) in bucket.iter().enumerate() {
            if tuple.k == key {
                key_pos = Some(pos);
            }
        }

        key_pos.map(|pos| {
            self.size -= 1;
            bucket.remove(pos).v
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl Hash for String {
    fn hash(&self) -> i64 {
        let mut result = 0;
        for c in self.bytes() {
            result = (result << 5) + result + (c as i64);
        }
        result
    }
}

impl Hash for i64 {
    fn hash(&self) -> i64 {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let mut map: HashTable<String, i64> = HashTable::new();

        assert!(map.is_empty());
        assert_eq!(map.get("AAA".to_owned()), None);

        map.put("ABC".to_owned(), 1);
        assert_eq!(map.size(), 1);
        assert_eq!(map.get("ABC".to_owned()), Some(&1));

        map.put("BBB".to_owned(), 2);
        assert_eq!(map.size(), 2);
        assert_eq!(map.get("BBB".to_owned()), Some(&2));

        map.put("ABC".to_owned(), 3);
        assert_eq!(map.size(), 2);
        assert_eq!(map.get("ABC".to_owned()), Some(&3));
        assert_eq!(map.get("BBB".to_owned()), Some(&2));

        map.put("BBB".to_owned(), 4);
        assert_eq!(map.size(), 2);
        assert_eq!(map.get("ABC".to_owned()), Some(&3));
        assert_eq!(map.get("BBB".to_owned()), Some(&4));

        assert_eq!(map.delete("BBB".to_owned()), Some(4));
        assert_eq!(map.size(), 1);
        assert_eq!(map.get("ABC".to_owned()), Some(&3));
        assert_eq!(map.get("BBB".to_owned()), None);

        assert_eq!(map.delete("ABC".to_owned()), Some(3));
        assert_eq!(map.size(), 0);
        assert_eq!(map.get("ABC".to_owned()), None);
        assert_eq!(map.get("BBB".to_owned()), None);

        assert_eq!(map.delete("ABC".to_owned()), None);
        assert_eq!(map.delete("BBB".to_owned()), None);
    }

    #[test]
    pub fn test_rehash() {
        let mut map: HashTable<i64, i64> = HashTable::new();

        for i in 0 as i64..100000 {
            map.put(i, i);
        }

        for i in 0 as i64..100000 {
            assert_eq!(map.get(i), Some(&i));
            assert_eq!(map.delete(i), Some(i));
            assert_eq!(map.size(), (100000 - 1 - i) as usize);
        }

        assert!(map.is_empty());
    }
}
