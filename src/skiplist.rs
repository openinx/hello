pub struct SkipList<K, V> {
    size: usize,
    heads: Vec<Link<K, V>>,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K, V> {
    k: K,
    v: V,
    next: Option<Box<Node<K, V>>>,
}

pub struct Iter<'a, K, V> {
    cur: Option<&'a Node<K, V>>,
}

impl<K, V> SkipList<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        SkipList {
            size: 0,
            heads: Vec::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> bool {
        todo!()
    }

    pub fn get(&self, k: K) -> Option<&V> {
        todo!()
    }

    pub fn delete(&mut self, k: K) -> Option<V> {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }

    pub fn iter<'a>(&self) -> Iter<'a, K, V> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_basics() {}
}
