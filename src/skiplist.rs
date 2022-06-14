use rand::Rng;
use std::cmp::{max, Ordering};

pub struct SkipList<K, V> {
    size: usize,
    level: usize,
    head: Link<K, V>,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K, V> {
    entry: Option<Entry<K, V>>,
    next: Vec<Link<K, V>>,
}

pub struct Entry<K, V> {
    k: K,
    v: V,
}

pub struct Iter<'a, K, V> {
    cur: &'a Link<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new_head(level: usize) -> Self {
        Node {
            entry: None,
            next: (0..level).map(|_| None).collect(),
        }
    }

    pub fn new_data(k: K, v: V, level: usize) -> Self {
        Node {
            entry: Some(Entry { k, v }),
            next: (0..level).map(|_| None).collect(),
        }
    }
}

impl<K, V> SkipList<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        SkipList {
            size: 0,
            level: 0,
            head: Some(Box::new(Node::new_head(0))),
        }
    }

    fn rand_level(&self) -> usize {
        let mut level = 1;
        let mut rng = rand::thread_rng();
        while rng.gen::<f32>() < 0.5 {
            level += 1;
        }
        // The max level is 16.
        std::cmp::min(level, 16)
    }

    pub fn insert(&mut self, k: K, v: V) -> bool {
        return true;
    }

    pub fn get(&self, k: K) -> Option<&V> {
        todo!()
    }

    pub fn delete(&mut self, k: K) -> Option<V> {
        todo!()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        todo!()
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a Entry<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    pub fn test_basics() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            println!("{}", rng.gen::<f32>());
        }
    }
}
