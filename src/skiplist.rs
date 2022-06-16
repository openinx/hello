use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Display;
use std::rc::Rc;

pub struct SkipList<K, V> {
    size: usize,
    level: usize,
    head: Link<K, V>,
}

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

pub struct Node<K, V> {
    entry: Option<Entry<K, V>>,
    forward: Vec<Link<K, V>>,
}

pub struct Entry<K, V> {
    k: K,
    v: V,
}

pub struct Iter<K, V> {
    cur: Link<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(k: K, v: V, level: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            entry: Some(Entry { k, v }),
            forward: (0..level).map(|_| None).collect(),
        }))
    }

    // Initialize a dummy node with None entry for the head node.
    pub fn new_head(level: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            entry: None,
            forward: (0..level).map(|_| None).collect(),
        }))
    }

    pub fn key(&self) -> &K {
        &self.entry.as_ref().unwrap().k
    }

    pub fn val(&self) -> &V {
        &self.entry.as_ref().unwrap().v
    }

    pub fn resize(&mut self, new_level: usize) {
        self.forward.resize(new_level, None);
    }

    pub fn level(&self) -> usize {
        return self.forward.capacity();
    }
}

impl<K, V> SkipList<K, V>
where
    K: Ord,
    V: Clone, // TODO Find the correct approach in self.get(..) to estimate the Clone trait.
{
    pub fn new() -> Self {
        SkipList {
            size: 0,
            level: 1,
            head: Some(Node::new_head(1)),
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

    fn get_prec(&self, mut ptr: Link<K, V>, level: usize, k: &K) -> Link<K, V> {
        while let Some(node) = ptr.clone() {
            let mut is_prec = false;
            match node.borrow().forward[level].as_ref() {
                None => is_prec = true,
                Some(next) if k <= next.borrow().key() => {
                    is_prec = true;
                }
                _ => {}
            }

            if is_prec {
                return ptr;
            } else {
                ptr = node.borrow().forward[level].clone();
            }
        }

        None
    }

    pub fn put(&mut self, k: K, v: V) {
        let new_level = self.rand_level();
        let new_node = Node::new(k, v, new_level);

        let mut ptr = self.head.clone();
        for i in (0..std::cmp::min(self.level, new_level)).rev() {
            ptr = self.get_prec(ptr, i, &new_node.borrow().key());

            ptr.as_ref().map(|pre_node| {
                let mut pre_mut_ref = pre_node.borrow_mut();
                match pre_mut_ref.forward[i].take() {
                    None => {
                        pre_mut_ref.forward[i] = Some(new_node.clone());
                        new_node.borrow_mut().forward[i] = None;
                    }
                    Some(next_node) => {
                        pre_mut_ref.forward[i] = Some(new_node.clone());
                        new_node.borrow_mut().forward[i] = Some(next_node);
                    }
                }
            });
        }

        if self.level < new_level {
            self.head.as_mut().map(|head_node| {
                let mut head_node_ref = head_node.borrow_mut();
                head_node_ref.resize(new_level);
                for i in self.level..new_level {
                    head_node_ref.forward[i] = Some(new_node.clone());
                }
            });

            self.level = new_level;
        }

        self.size += 1;
    }

    pub fn get(&self, k: K) -> Option<V> {
        let mut ptr = self.head.clone();
        for i in (0..self.level).rev() {
            // Iterate to find the correct key in the leve-i.
            while let Some(node) = ptr.clone() {
                match node.borrow().forward[i].clone() {
                    None => {
                        // Don't have more key in this level-i.
                        break;
                    }
                    Some(next) => {
                        match Ord::cmp(&k, next.borrow().key()) {
                            Ordering::Less => {
                                // Cann't find the key in current level! Let's just goto the next level.
                                break;
                            }
                            Ordering::Equal => {
                                // Find the correct key value in currect level.
                                return Some(next.borrow().val().clone());
                            }
                            Ordering::Greater => {
                                // Iterate to the next key in current level.
                                ptr = node.borrow().forward[i].clone();
                            }
                        }
                    }
                }
            }
        }

        // The target key is greater than all of the keys in the collection.
        return None;
    }

    pub fn delete(&mut self, k: K) -> Option<V> {
        let mut ptr = self.head.clone();
        for i in (0..self.level).rev() {
            // Iterate to find the correct key in the leve-i.
            while let Some(node) = ptr.clone() {
                let mut prev_mut_ref = node.borrow_mut();
                match prev_mut_ref.forward[i].clone() {
                    None => {
                        // Don't have more key in this level-i.
                        break;
                    }
                    Some(next) => {
                        let mut next_mut_ref = next.borrow_mut();
                        match Ord::cmp(&k, next_mut_ref.key()) {
                            Ordering::Less => {
                                // Cann't find the key in current level! Let's just goto the next level.
                                break;
                            }
                            Ordering::Equal => {
                                prev_mut_ref.forward[i].take().map(|_| {
                                    prev_mut_ref.forward[i] = next_mut_ref.forward[i].take();
                                });

                                // Find the correct key value in currect level.
                                if i <= 0 {
                                    self.size -= 1;
                                    return Some(next_mut_ref.val().clone());
                                }
                            }
                            Ordering::Greater => {
                                // Iterate to the next key in current level.
                                ptr = prev_mut_ref.forward[i].clone();
                            }
                        }
                    }
                }
            }
        }

        // The target key is not existing in the collection.
        None
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn peek_front(&self) -> Link<K, V> {
        match self.head.clone() {
            None => None,
            Some(head_node) => head_node.borrow().forward[0].clone(),
        }
    }

    pub fn iter(&self) -> Iter<K, V> {
        return Iter {
            cur: match self.head.clone() {
                None => None,
                Some(head_node) => head_node.borrow().forward[0].clone(),
            },
        };
    }
}

impl<K, V> Display for SkipList<K, V>
where
    K: Display + Ord,
    V: Display + Clone,
{
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;

        // Get the first node by skipping the dummy head node.
        let mut node = self.peek_front();

        while let Some(n) = node {
            write!(w, "<{},{}>", n.borrow().key(), n.borrow().val())?;
            node = n.borrow().forward[0].clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}

// TODO Implement a generic Iterator without any cloning ?
// TODO To be frank, I still don't get the correct approach to handle the lifetime issues
// TODO by implementing this iterator with zero-copy.
impl<K, V> Iterator for Iter<K, V>
where
    K: Clone,
    V: Clone,
{
    type Item = Entry<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.take().map(|node| {
            let t = node.borrow();
            self.cur = t.forward[0].clone();

            Entry {
                k: t.key().clone(),
                v: t.val().clone(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::SliceRandom;

    #[test]
    pub fn test_basics() {
        let mut sorted_map = SkipList::new();

        let mut rng = rand::thread_rng();

        let max = 1000_0 as usize;
        let mut data: Vec<usize> = (0..max).collect();
        data.shuffle(&mut rng);

        for i in 0..max {
            assert_eq!(sorted_map.get(data[i]), None);

            sorted_map.put(data[i], data[i]);

            assert_eq!(sorted_map.get(data[i]), Some(data[i]));
            assert_eq!(sorted_map.size(), i + 1);
        }

        for i in 0..max {
            assert_eq!(sorted_map.get(data[i]), Some(data[i]));
        }

        let mut iter = sorted_map.iter();
        for i in 0..max {
            match iter.next() {
                None => panic!("Expected to have more entries."),
                Some(e) => {
                    assert_eq!(e.k, i);
                    assert_eq!(e.v, i);
                }
            }
        }
    }

    #[test]
    pub fn test_delete() {
        let mut sorted_map = SkipList::new();
        let mut rng = rand::thread_rng();
        let max = 1000_0 as usize;
        let mut data: Vec<usize> = (0..max).collect();
        data.shuffle(&mut rng);

        for i in 0..max {
            sorted_map.put(data[i], data[i]);
            assert_eq!(sorted_map.get(data[i]), Some(data[i]));
            assert_eq!(sorted_map.size(), i + 1);
        }

        {
            let mut hit = vec![false; max];
            let mut iter = sorted_map.iter();
            while let Some(e) = iter.next() {
                assert_eq!(e.k, e.v);
                assert_eq!(hit[e.k], false);
                hit[e.k] = true;
            }

            assert_eq!(hit, vec![true; max]);
        }

        for i in 0..max {
            assert_eq!(sorted_map.delete(data[i]), Some(data[i]));
            assert_eq!(sorted_map.get(data[i]), None);
            assert_eq!(sorted_map.size(), max - 1 - i);
        }

        {
            let mut iter = sorted_map.iter();
            while let Some(_) = iter.next() {
                panic!("Expect to have no elements in this collection.");
            }
            assert_eq!(0, sorted_map.size());
        }
    }

    #[test]
    pub fn test_debug() {
        let mut sorted_map = SkipList::new();
        assert_eq!("[]", format!("{}", sorted_map));

        sorted_map.put("A", 3);
        sorted_map.put("B", 2);
        sorted_map.put("C", 5);
        sorted_map.put("A", 6);
        assert_eq!("[<A,6>, <A,3>, <B,2>, <C,5>]", format!("{}", sorted_map));

        assert_eq!(sorted_map.delete("A"), Some(6));
        assert_eq!("[<A,3>, <B,2>, <C,5>]", format!("{}", sorted_map));
        assert_eq!(sorted_map.delete("A"), Some(3));
        assert_eq!("[<B,2>, <C,5>]", format!("{}", sorted_map));
        assert_eq!(sorted_map.delete("A"), None);
        assert_eq!("[<B,2>, <C,5>]", format!("{}", sorted_map));

        assert_eq!(sorted_map.delete("B"), Some(2));
        assert_eq!("[<C,5>]", format!("{}", sorted_map));
        assert_eq!(sorted_map.delete("B"), None);
        assert_eq!("[<C,5>]", format!("{}", sorted_map));

        assert_eq!(sorted_map.delete("C"), Some(5));
        assert_eq!("[]", format!("{}", sorted_map));
        assert_eq!(sorted_map.delete("C"), None);
        assert_eq!("[]", format!("{}", sorted_map));
    }
}
