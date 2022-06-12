pub struct AVLTree<K, V> {
    size: usize,
    root: Option<Box<Node<K, V>>>,
}

pub struct Node<K, V> {
    k: K,
    v: V,
    l: Option<Box<Node<K, V>>>,
    r: Option<Box<Node<K, V>>>,
}

pub struct Iter<'a, K, V> {
    cur: Option<Box<&'a Node<K, V>>>,
}

impl<K, V> AVLTree<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        AVLTree {
            size: 0,
            root: None,
        }
    }

    pub fn size(&self) -> usize {
        todo!()
    }

    pub fn insert(&mut self, key: K, val: V) -> bool {
        todo!()
    }

    pub fn delete(&mut self, key: K) -> Option<V> {
        todo!()
    }

    pub fn get(&self, key: K) -> Option<&V> {
        todo!()
    }

    pub fn prec(&self, key: K) -> Option<&V> {
        todo!()
    }

    pub fn succ(&self, key: K) -> Option<&V> {
        todo!()
    }

    pub fn height(&self) -> usize {
        todo!()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_basics() {}
}
