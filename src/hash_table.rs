pub struct HashTable<K, V> {
    size: i32,
    buckets: Vec<Bucket<K, V>>,
}

struct Tuple<K, V> {
    k: K,
    v: V,
}

struct Bucket<K, V> {
    elems: Vec<Tuple<K, V>>,
}

impl<K, V> HashTable<K, V> {
    pub fn new() -> HashTable<K, V> {
        HashTable {
            size: 0,
            buckets: Vec::with_capacity(4),
        }
    }

    pub fn put(&mut self, key: K, val: V) {}

    pub fn find(&mut self, key: K) -> Option<V> {
        None
    }

    pub fn delete(&mut self, key: K) -> Option<V> {
        None
    }

    pub fn size(&self) -> i32 {
        0
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let mut hash = HashTable::new();

        hash.put(1, 2);
        hash.size();
    }
}
