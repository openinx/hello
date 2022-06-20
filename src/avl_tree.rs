use std::cmp::Ordering;

pub struct AVLTree<K, V> {
    size: usize,
    root: Option<Box<Node<K, V>>>,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

// Balance factor enums.
pub enum BF {
    LeftHigh,
    RightHigh,
    Equal,
}

pub struct Node<K, V> {
    k: K,
    v: V,
    bf: BF, // Balance Factor.
    l: Option<Box<Node<K, V>>>,
    r: Option<Box<Node<K, V>>>,
}

trait TreeNode<K, V>
where
    K: Ord,
{
    /**
     * Right rotate the tree :
     *             (A)
     *             /
     *           (B)        =>       (B)
     *           /                  /   \
     *         (C)                (C)   (A)
     */
    fn right_rotate(&mut self);

    /**
     *  Left rotate the tree:
     *          (A)
     *            \
     *            (B)       =>       (B)
     *              \               /   \
     *              (C)           (A)   (C)
     */
    fn left_rotate(&mut self);

    /**
     *  Balance the right side after insert a new key-value.
     *  If the new node is added as the following, then do the right_rotate(A):
     *
     *             (A)
     *               \
     *               (B)    =>         (B)
     *                 \              /   \
     *                (New)         (A)   (New)
     *
     *  Otherwise do the right_rotate(B) and left_rotate(A):
     *
     *              (A)          (A)
     *                \            \
     *                (B)   =>     (New)  =>     (New)
     *                /              \           /   \
     *              (New)            (B)       (A)   (B)
     *
     */
    fn right_balance(&mut self);

    /**
     *  Balance the left side tree after insert a new key-value.
     *  If the new node is added as the following, then do the left_rotate(A):
     *
     *             (A)
     *             /
     *           (B)        =>       (B)
     *           /                  /   \
     *         (New)             (New)   (A)
     *
     *  Otherwise do the left_rotate(B) and right_rotate(A):
     *
     *             (A)            (A)
     *             /              /
     *           (B)      =>    (New)   =>   (New)
     *             \            /            /   \
     *            (New)       (B)          (B)   (A)
     */
    fn left_balance(&mut self);

    fn get(&self, k: K) -> Option<&V>;

    // Get the height of balanced binary tree. It will panic if find any invalid
    // balance factor during visiting the whole tree.
    fn height(&self) -> usize;

    // Insert a (key, val) pair into the tree.
    // The 1th returned bool indicate whether the pair is inserted or not.
    // The 2nd returned bool indicate whether the tree has been taller after the insertion.
    fn add(&mut self, k: K, v: V) -> (bool, bool);
}

impl<K, V> TreeNode<K, V> for Link<K, V>
where
    K: Ord,
{
    fn right_rotate(&mut self) {
        let mut l_child = self.as_mut().unwrap().l.take();
        self.as_mut().unwrap().l = l_child.as_mut().unwrap().r.take();
        l_child.as_mut().unwrap().r = self.take();
        *self = l_child;
    }

    fn left_rotate(&mut self) {
        let mut r_child = self.as_mut().unwrap().r.take();
        self.as_mut().unwrap().r = r_child.as_mut().unwrap().l.take();
        r_child.as_mut().unwrap().l = self.take();
        *self = r_child;
    }

    fn right_balance(&mut self) {
        let right_child_bf = &self.as_ref().unwrap().r.as_ref().unwrap().bf;
        match right_child_bf {
            BF::LeftHigh => {
                let left_bf_of_r_child = &self
                    .as_ref()
                    .unwrap()
                    .r
                    .as_ref()
                    .unwrap()
                    .l
                    .as_ref()
                    .unwrap()
                    .bf;
                match left_bf_of_r_child {
                    BF::LeftHigh => {
                        self.as_mut().unwrap().bf = BF::Equal;
                        self.as_mut().unwrap().r.as_mut().unwrap().bf = BF::RightHigh;
                    }
                    BF::Equal => {
                        self.as_mut().unwrap().bf = BF::Equal;
                        self.as_mut().unwrap().r.as_mut().unwrap().bf = BF::Equal;
                    }
                    BF::RightHigh => {
                        self.as_mut().unwrap().bf = BF::LeftHigh;
                        self.as_mut().unwrap().r.as_mut().unwrap().bf = BF::Equal;
                    }
                }

                self.as_mut()
                    .unwrap()
                    .r
                    .as_mut()
                    .unwrap()
                    .l
                    .as_mut()
                    .unwrap()
                    .bf = BF::Equal;
                self.as_mut().unwrap().r.right_rotate();
                self.left_rotate();
            }
            BF::RightHigh => {
                self.as_mut().unwrap().bf = BF::Equal;
                self.as_mut().unwrap().r.as_mut().unwrap().bf = BF::Equal;
                self.left_rotate();
            }
            _ => {}
        }
    }

    fn left_balance(&mut self) {
        let left_child_bf = &self.as_ref().unwrap().l.as_ref().unwrap().bf;
        match left_child_bf {
            BF::LeftHigh => {
                self.as_mut().unwrap().bf = BF::Equal;
                self.as_mut().unwrap().l.as_mut().unwrap().bf = BF::Equal;
                self.right_rotate();
            }
            BF::RightHigh => {
                let right_bf_of_l_child = &self
                    .as_ref()
                    .unwrap()
                    .l
                    .as_ref()
                    .unwrap()
                    .r
                    .as_ref()
                    .unwrap()
                    .bf;
                match right_bf_of_l_child {
                    BF::LeftHigh => {
                        self.as_mut().unwrap().bf = BF::RightHigh;
                        self.as_mut().unwrap().l.as_mut().unwrap().bf = BF::Equal;
                    }
                    BF::Equal => {
                        self.as_mut().unwrap().bf = BF::Equal;
                        self.as_mut().unwrap().l.as_mut().unwrap().bf = BF::Equal;
                    }
                    BF::RightHigh => {
                        self.as_mut().unwrap().bf = BF::Equal;
                        self.as_mut().unwrap().l.as_mut().unwrap().bf = BF::LeftHigh;
                    }
                }

                self.as_mut()
                    .unwrap()
                    .l
                    .as_mut()
                    .unwrap()
                    .r
                    .as_mut()
                    .unwrap()
                    .bf = BF::Equal;
                self.as_mut().unwrap().l.left_rotate();
                self.right_rotate();
            }
            _ => {}
        }
    }

    fn height(&self) -> usize {
        match &self {
            None => 0,
            Some(node) => {
                let l_height = node.l.height();
                let r_height = node.r.height();

                // Do the verification.
                assert!((l_height as i32 - r_height as i32).abs() <= 1);
                return std::cmp::max(l_height, r_height) + 1;
            }
        }
    }

    fn get(&self, k: K) -> Option<&V> {
        match self {
            None => None,
            Some(node) => match Ord::cmp(&k, &node.k) {
                Ordering::Less => node.l.get(k),
                Ordering::Equal => Some(&node.v),
                Ordering::Greater => node.r.get(k),
            },
        }
    }

    fn add(&mut self, k: K, v: V) -> (bool, bool) {
        if self.is_none() {
            *self = Some(Box::new(Node {
                k,
                v,
                bf: BF::Equal,
                l: None,
                r: None,
            }));
            return (
                true, /* Inserted a new node. */
                true, /* It's more taller because of the successful insertion. */
            );
        }

        match Ord::cmp(&k, &self.as_ref().unwrap().k) {
            // Case.1: k < *key
            Ordering::Less => {
                let (inserted, mut taller) = self.as_mut().unwrap().l.add(k, v);
                if !inserted {
                    return (false, false);
                }
                if !taller {
                    return (true, false);
                }

                match self.as_deref().unwrap().bf {
                    BF::LeftHigh => {
                        self.left_balance();
                        taller = false;
                    }
                    BF::RightHigh => {
                        self.as_mut().unwrap().bf = BF::Equal;
                        taller = false;
                    }
                    BF::Equal => {
                        self.as_mut().unwrap().bf = BF::LeftHigh;
                        taller = true;
                    }
                }

                return (inserted, taller);
            }

            // Case.2: k == *key
            Ordering::Equal => {
                return (false, false);
            }

            // Case.3: k > *key
            Ordering::Greater => {
                let (inserted, mut taller) = self.as_deref_mut().unwrap().r.add(k, v);
                if !inserted {
                    return (false, false);
                }
                if !taller {
                    return (true, false);
                }

                match self.as_deref().unwrap().bf {
                    BF::LeftHigh => {
                        self.as_mut().unwrap().bf = BF::Equal;
                        taller = false;
                    }
                    BF::RightHigh => {
                        self.right_balance();
                        taller = false;
                    }
                    BF::Equal => {
                        self.as_mut().unwrap().bf = BF::RightHigh;
                        taller = true;
                    }
                }

                return (inserted, taller);
            }
        }
    }
}

pub struct Iter<'a, K, V> {
    stack: Vec<&'a Node<K, V>>,
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
        self.size
    }

    pub fn insert(&mut self, key: K, val: V) -> bool {
        let (succ, _) = self.root.add(key, val);
        if succ {
            self.size += 1;
        }
        succ
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.root.get(key)
    }

    pub fn height(&self) -> usize {
        self.root.height()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        let mut stack = Vec::new();

        let mut cur = &self.root;
        while let Some(node) = cur.as_deref() {
            stack.push(node);
            cur = &node.l;
        }

        Iter { stack }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(node) => {
                // Enqueue left path of the first right child.
                let mut cur = &node.r;
                while let Some(cur_node) = cur.as_deref() {
                    self.stack.push(cur_node);
                    cur = &cur_node.l;
                }

                // Just return the pop node from the stack.
                Some(node)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rand;

    #[test]
    pub fn test_basics() {
        let mut tree = AVLTree::new();

        let max = 1000_000;
        let mut input: Vec<u32> = (0..max).collect();
        rand::shuffle(&mut input);

        for i in 0..max as usize {
            assert_eq!(tree.insert(input[i], input[i]), true);
            assert_eq!(tree.size(), i + 1);
        }
        tree.height();

        let mut hit: Vec<bool> = vec![false; max as usize];
        for i in 0..max {
            assert_eq!(tree.get(i), Some(&i));
            assert_eq!(hit[i as usize], false);
            hit[i as usize] = true;
        }

        assert_eq!(hit, vec![true; max as usize]);

        {
            let mut iter = tree.iter();

            for i in 0..max {
                match iter.next() {
                    None => panic!("We're expected to read more element from the iter."),
                    Some(node) => {
                        assert_eq!(node.k, i);
                        assert_eq!(node.v, i);
                    }
                }
            }

            assert_eq!(iter.next().is_none(), true);
        }
    }

    #[test]
    pub fn test_verify_tree() {
        let max = 1000;
        let mut tree = AVLTree::new();

        for _ in 0..max {
            let x = rand::gen_i32();
            tree.insert(x, x);
            tree.height();
        }
    }
}
