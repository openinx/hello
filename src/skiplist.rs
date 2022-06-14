use std::cmp::Ordering;

use rand::Rng;

const MAX_LEVEL: usize = 16;

pub struct SkipList<K, V> {
    size: usize,
    level: usize,
    head: Link<K, V>,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K, V> {
    k: K,
    v: V,
    next: Vec<Link<K, V>>,
}

pub struct Iter<'a, K, V> {
    cur: &'a Link<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(k: K, v: V, level: usize) -> Self {
        Node {
            k,
            v,
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
            head: None,
        }
    }

    fn rand_level() -> usize {
        let mut level = 0;
        let mut rng = rand::thread_rng();
        while rng.gen::<f32>() < 0.5 {
            level += 1;
        }
        std::cmp::min(level, MAX_LEVEL)
    }

    pub fn insert(&mut self, k: K, v: V) -> bool {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(k, v, 1)));
            self.size += 1;
            self.level = 1;
        }

        todo!()
    }

    pub fn get(&self, k: K) -> Option<&V> {
        let mut ptr = &self.head;
        let mut cur_level = std::cmp::max(self.level, 1) - 1;
        while let Some(node) = ptr {
            match Ord::cmp(&k, &node.k) {
                Ordering::Less => {
                    if cur_level <= 0 {
                        return None;
                    }
                    cur_level -= 1;
                    ptr = &node.next[cur_level];
                }
                Ordering::Equal => {
                    return Some(&node.v);
                }
                Ordering::Greater => {
                    ptr = &node.next[cur_level];
                }
            }
        }
        None
    }

    pub fn delete(&mut self, k: K) -> Option<V> {
        todo!()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter { cur: &self.head }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.as_deref().map(|node| {
            self.cur = &node.next[0];
            node
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_basics() {}
}
