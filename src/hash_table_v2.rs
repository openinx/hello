pub trait Hash: Eq {
    fn hash(&self) -> i64;
}

// Use linked list to handle the hash conflicts for the same bucket.
pub struct HashMap<K: Hash, V> {
    size: usize,
    buckets: Vec<Ptr<K, V>>,
}

type Ptr<K, V> = Option<Box<Node<K, V>>>;
pub struct Node<K: Hash, V> {
    key: K,
    val: V,
    next: Ptr<K, V>,
}

pub struct Iter<'a, K: Hash, V> {
    index: usize,
    cur: &'a Ptr<K, V>,
}

impl<K, V> HashMap<K, V>
where
    K: Hash,
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn put(&mut self, key: K, val: V) {
        todo!()
    }

    pub fn get(&self, key: K) -> Option<&V> {
        todo!()
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        todo!()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter {
            index: 0,
            cur: &None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {}
}
