use rand::distributions::uniform::SampleBorrow;
use rand::Rng;
use std::cell::RefCell;
use std::cmp::{max, Ordering};
use std::rc::Rc;

pub struct SkipList<K, V> {
    size: usize,
    level: usize,
    head: Link<K, V>,
}

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

pub struct Node<K, V> {
    k: K,
    v: V,
    forward: Vec<Link<K, V>>,
}

pub struct Iter<'a, K, V> {
    cur: &'a Link<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(k: K, v: V, level: usize) -> Self {
        Node {
            k,
            v,
            forward: (0..level).map(|_| None).collect(),
        }
    }

    pub fn level(&self) -> usize {
        return self.forward.capacity();
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

    fn rand_level(&self) -> usize {
        let mut level = 1;
        let mut rng = rand::thread_rng();
        while rng.gen::<bool>() {
            level += 1;
        }
        // The max level is 16.
        std::cmp::min(level, 16)
    }

    fn less_than_next(&self, node: Rc<RefCell<Node<K, V>>>, level: usize, k: &K) -> bool {
        let mut ptr = node.clone();
        while let Some(next) = &ptr.borrow().forward[level] {
            if k < &next.borrow().k {
                return true;
            }
        }
        return false;
    }

    pub fn insert(&mut self, k: K, v: V) {
        if self.head.is_none() {
            self.head = Some(Rc::new(RefCell::new(Node::new(k, v, 1))));
            self.size += 1;
            self.level += 1;
            return;
        }

        // let new_node = Some(Rc::new(RefCell::new(Node::new(k, v, self.rand_level()))));

        let mut node = self.head.clone();
        for i in (0..self.level).rev() {

            // loop {
            //     let next_ptr = node.unwrap().borrow().forward[i].clone();
            //     if next_ptr.is_some() && k >= next_ptr.unwrap().borrow().k {
            //         node = next_ptr.clone();
            //     } else {
            //         break;
            //     }
            // }

            // while node.borrow().forward[i]

            // while let Some(cur) = ptr {
            //     if let Some(next) = &cur.borrow().forward[i] {
            //         if k < next.borrow().k {
            //             break;
            //         }
            //     }
            // }
        }
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
    type Item = &'a Node<K, V>;

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
