use rand::Rng;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::cmp::{Ordering, max};
use std::rc::Rc;

const MAX_LEVEL: usize = 16;

pub struct SkipList<K, V> {
    size: usize,
    level: usize,
    head: Link<K, V>,
}

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

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

    fn rand_level(&self) -> usize {
        let mut level = 1;
        let mut rng = rand::thread_rng();
        while rng.gen::<f32>() < 0.5 {
            level += 1;
        }
        std::cmp::min(level, MAX_LEVEL)
    }

    pub fn insert(&mut self, k: K, v: V) -> bool {
        if self.head.is_none() {
            self.head = Some(Rc::new(RefCell::new(Node::new(k, v, 1))));
            self.size += 1;
            self.level = 1;
            return true;
        }

        let res = Ord::cmp(&k, &self.head.as_deref().unwrap().borrow().k);
        match res {
            Ordering::Less => {
                // Replace the head with the new one if it's inserted in the 1th place.

                // The new node's level is always >= the head node's level.
                let new_level = max(self.rand_level(), self.level);
                let mut new_node = Node::new(k, v, new_level);

                self.head.take().map(|old_head|{
                    for l in 0..self.level {
                        // Clone a new ref to the new_node.next[l]
                        new_node.next[l] = Some(old_head.clone());
                    }

                    self.head = Some(Rc::new(RefCell::new(new_node)));
                });

                self.level = new_level;
                self.size += 1;
                return true;
            }
            Ordering::Equal => {
                // self.head.as_mut().map(|head| {
                //     head.borrow_mut();
                // });
                //self.head.unwrap().borrow_mut();
            }
            Ordering::Greater => {}
        }

        todo!()
    }

    pub fn get(&self, k: K) -> Option<&V> {
        todo!()
        // let mut ptr = &self.head;
        // let mut cur_level = std::cmp::max(self.level, 1) - 1;
        // while let Some(node) = ptr {
        //     match Ord::cmp(&k, &node.k) {
        //         Ordering::Less => {
        //             if cur_level <= 0 {
        //                 return None;
        //             }
        //             cur_level -= 1;
        //             ptr = &node.next[cur_level];
        //         }
        //         Ordering::Equal => {
        //             return Some(&node.v);
        //         }
        //         Ordering::Greater => {
        //             ptr = &node.next[cur_level];
        //         }
        //     }
        // }
        // None
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

// impl<'a, K, V> Iterator for Iter<'a, K, V> {
//     type Item = Ref<&'a Node<K, V>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.cur.as_ref().map(|node| {
//             Ref::map(node.borrow(), |node|{ &node.k});
//             // self.cur = &node.borrow().next[0];
//             // node.borrow()
//         })
//     }
// }

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
