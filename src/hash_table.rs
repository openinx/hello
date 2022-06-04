pub trait Key: Eq {
    fn hash(&self) -> i64;
}

pub struct HashTable<K, V> {
    size: usize,
    buckets: Vec<Bucket<K, V>>,
}

struct Tuple<K, V> {
    k: K,
    v: V,
}

struct Bucket<K, V> {
    elems: Vec<Tuple<K, V>>,
}

impl<K, V> HashTable<K, V>
where
    K: Key,
{
    pub fn new() -> HashTable<K, V> {
        HashTable {
            size: 0,
            buckets: Vec::with_capacity(4),
        }
    }

    fn bucket_idx(&self, key: &K) -> usize {
        let hash_code = key.hash();
        let bucket_size = self.buckets.len() as i64;
        ((hash_code % bucket_size + bucket_size) % bucket_size) as usize
    }

    pub fn put(&mut self, key: K, val: V) {
        todo!()
    }

    pub fn find(&mut self, key: K) -> Option<V> {
        None
    }

    pub fn delete(&mut self, key: K) -> Option<V> {
        None
    }

    pub fn length(&self) -> i32 {
        0
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {}
}
