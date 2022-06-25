use std::ptr;

use crate::rand;

pub struct SkipList<K, V> {
    size: usize,
    level: usize,
    head: Vec<*mut Node<K, V>>,
}

pub struct Node<K, V> {
    entry: Option<Entry<K, V>>,
    next: Vec<*mut Node<K, V>>,
}

pub struct Entry<K, V> {
    k: K,
    v: V,
}

impl<K, V> Node<K, V> {
    pub fn new(level: usize, k: K, v: V) -> Self {
        Node {
            entry: Some(Entry { k, v }),
            next: (0..level).map(|_| ptr::null_mut()).collect(),
        }
    }

    pub fn key(&self) -> &K {
        &self.entry.as_ref().unwrap().k
    }

    pub fn val(&self) -> &V {
        &self.entry.as_ref().unwrap().v
    }
}

impl<K, V> SkipList<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        SkipList {
            size: 0,
            level: 1,
            head: (0..1).map(|_| std::ptr::null_mut()).collect(),
        }
    }

    fn rand_level(&self) -> usize {
        let mut level = 1;
        while rand::gen_bool() {
            level += 1;
        }
        return level;
    }

    pub fn add(&mut self, k: K, v: V) {
        unsafe {
            let mut ptr = self.head[self.level - 1];
            let new_node = Node::new(self.rand_level(), k, v);
            let min_level = std::cmp::min(new_node.next.len(), self.level);

            for level in (0..min_level).rev() {
                // Find the correct ptr to link the new node.
                while !(*ptr).next[level].is_null() {
                    let next = Box::from_raw((*ptr).next[level]);
                    if new_node.key() < next.key() {
                        break;
                    } else {
                        ptr = next.next[level];
                    }
                }

                //*((*ptr).next[level]) = new_node;
            }
        }
    }

    pub fn delete(&mut self, k: K, v: V) {
        todo!()
    }

    pub fn get(&self, k: K) -> Option<V> {
        unsafe {
            let mut ptr = self.head[self.level - 1];
            for level in (0..self.level).rev() {
                while !(*ptr).next[level].is_null() {
                    let next = Box::from_raw((*ptr).next[level]);
                    if &k < next.key() {
                        break;
                    } else if &k == next.key() {
                        return Some((*next).entry.unwrap().v);
                    } else {
                        ptr = (*ptr).next[level];
                    }
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn basic() {}
}
