use std::cmp;
pub struct SimpleTree<T: Ord> {
    size: usize,
    root: Option<Box<Node<T>>>,
}

pub struct Iter<'a, T: Ord> {
    stack: Vec<&'a Node<T>>,
}

struct Node<T: Ord> {
    elem: T,
    l: Option<Box<Node<T>>>,
    r: Option<Box<Node<T>>>,
}

impl<T> SimpleTree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        SimpleTree {
            size: 0,
            root: None,
        }
    }

    pub fn insert(&mut self, elem: T) {
        let mut cur = &mut self.root;
        loop {
            match cur {
                None => {
                    *cur = Some(Box::new(Node {
                        elem,
                        l: None,
                        r: None,
                    }));
                    self.size += 1;
                    break;
                }
                Some(node) => {
                    if elem < node.elem {
                        cur = &mut node.l;
                    } else if elem > node.elem {
                        cur = &mut node.r;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn find(&self, elem: T) -> Option<&T> {
        let mut cur = &self.root;
        loop {
            match cur {
                None => return None,
                Some(node) => {
                    if elem < node.elem {
                        cur = &node.l;
                    } else if elem > node.elem {
                        cur = &node.r;
                    } else {
                        return Some(&node.elem);
                    }
                }
            }
        }
    }

    pub fn max(&self) -> Option<&T> {
        let mut cur = &self.root;
        loop {
            match cur {
                None => return None,
                Some(node) => {
                    if node.r.is_none() {
                        return Some(&node.elem);
                    } else {
                        cur = &node.r;
                    }
                }
            }
        }
    }

    pub fn min(&self) -> Option<&T> {
        let mut cur = &self.root;
        loop {
            match cur {
                None => return None,
                Some(node) => {
                    if node.l.is_none() {
                        return Some(&node.elem);
                    } else {
                        cur = &node.l;
                    }
                }
            }
        }
    }

    fn _height(&self, ptr: &Option<Box<Node<T>>>) -> usize {
        match ptr.as_deref() {
            None => 0,
            Some(node) => cmp::max(self._height(&node.l), self._height(&node.r)) + 1,
        }
    }

    pub fn height(&self) -> usize {
        self._height(&self.root)
    }

    // Get the precursor element of the given element.
    pub fn prec(&self, elem: T) -> Option<&T> {
        let mut iter = self.iter();
        let mut prec = None;
        while let Some(cur) = iter.next() {
            if *cur < elem {
                prec = Some(cur);
            }
        }
        prec
    }

    // Get the succeed element of the given element.
    pub fn succ(&self, elem: T) -> Option<&T> {
        let mut iter = self.iter();
        while let Some(cur) = iter.next() {
            if *cur > elem {
                return Some(cur);
            }
        }
        return None;
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        let mut stack = Vec::new();
        let mut cur = &self.root;

        while cur.is_some() {
            stack.push(cur.as_deref().unwrap());
            cur = &cur.as_deref().unwrap().l;
        }

        Iter { stack }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<'a, T> Iter<'a, T>
where
    T: Ord,
{
    fn next(&mut self) -> Option<&'a T> {
        match self.stack.pop() {
            None => return None,
            Some(node) => {
                if node.r.is_some() {
                    // Push node.r and all left children of node.r into the stack.
                    let mut cur = &node.r;
                    while cur.is_some() {
                        self.stack.push(cur.as_deref().unwrap());
                        cur = &cur.as_deref().unwrap().l;
                    }
                }
                return Some(&node.elem);
            }
        };
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

fn internal_drop<T: Ord>(ptr: &mut Option<Box<Node<T>>>) {
    match ptr {
        None => (),
        Some(node) => {
            internal_drop(&mut node.l);
            internal_drop(&mut node.r);
            ptr.take();
        }
    }
}

impl<T> Drop for SimpleTree<T>
where
    T: Ord,
{
    fn drop(&mut self) {
        internal_drop(&mut self.root);
        self.size = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let mut tree = SimpleTree::new();

        tree.insert(1);
        assert_eq!(tree.find(1), Some(&1));
        assert_eq!(tree.find(2), None);
        assert_eq!(tree.find(3), None);
        assert_eq!(tree.size(), 1);
        assert_eq!(tree.max(), Some(&1));
        assert_eq!(tree.min(), Some(&1));
        assert_eq!(tree.height(), 1);

        tree.insert(2);
        assert_eq!(tree.find(1), Some(&1));
        assert_eq!(tree.find(2), Some(&2));
        assert_eq!(tree.find(3), None);
        assert_eq!(tree.size(), 2);
        assert_eq!(tree.max(), Some(&2));
        assert_eq!(tree.min(), Some(&1));
        assert_eq!(tree.height(), 2);

        tree.insert(3);
        assert_eq!(tree.find(1), Some(&1));
        assert_eq!(tree.find(2), Some(&2));
        assert_eq!(tree.find(3), Some(&3));
        assert_eq!(tree.size(), 3);
        assert_eq!(tree.max(), Some(&3));
        assert_eq!(tree.min(), Some(&1));
        assert_eq!(tree.height(), 3);
    }

    #[test]
    pub fn test_prec_succ() {
        let mut tree = SimpleTree::new();

        tree.insert(6);
        tree.insert(3);
        tree.insert(1);
        tree.insert(4);
        tree.insert(5);
        tree.insert(8);
        tree.insert(7);

        assert_eq!(tree.height(), 4);
        assert_eq!(tree.prec(-1), None);
        assert_eq!(tree.succ(-1), Some(&1));

        assert_eq!(tree.prec(1), None);
        assert_eq!(tree.succ(1), Some(&3));

        assert_eq!(tree.prec(2), Some(&1));
        assert_eq!(tree.succ(2), Some(&3));

        assert_eq!(tree.prec(3), Some(&1));
        assert_eq!(tree.succ(3), Some(&4));

        assert_eq!(tree.prec(4), Some(&3));
        assert_eq!(tree.succ(4), Some(&5));

        assert_eq!(tree.prec(5), Some(&4));
        assert_eq!(tree.succ(5), Some(&6));

        assert_eq!(tree.prec(6), Some(&5));
        assert_eq!(tree.succ(6), Some(&7));

        assert_eq!(tree.prec(7), Some(&6));
        assert_eq!(tree.succ(7), Some(&8));

        assert_eq!(tree.prec(8), Some(&7));
        assert_eq!(tree.succ(8), None);

        assert_eq!(tree.prec(9), Some(&8));
        assert_eq!(tree.succ(9), None);
    }

    #[test]
    pub fn test_iter() {
        let mut tree = SimpleTree::new();

        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        tree.insert(-1);
        tree.insert(5);
        tree.insert(4);

        {
            let mut iter = tree.iter();
            assert_eq!(iter.next(), Some(&-1));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), Some(&5));
            assert_eq!(iter.next(), None);
        }

        tree.insert(-2);
        tree.insert(9);

        {
            let mut iter = tree.iter();
            assert_eq!(iter.next(), Some(&-2));
            assert_eq!(iter.next(), Some(&-1));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), Some(&5));
            assert_eq!(iter.next(), Some(&9));
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    pub fn test_many_i64() {
        let mut tree = SimpleTree::new();

        for i in 0..10000 {
            tree.insert(i);
            assert_eq!(tree.height(), i + 1);
            assert_eq!(tree.find(i), Some(&i));
            assert_eq!(tree.min(), Some(&0));
            assert_eq!(tree.max(), Some(&i));
            assert_eq!(tree.size(), (i + 1) as usize);
        }
    }

    #[test]
    pub fn test_many_str() {
        let mut tree = SimpleTree::new();

        for i in 0..10000 {
            tree.insert(i.to_string());
            assert_eq!(tree.find(i.to_string()), Some(&i.to_string()));
            assert_eq!(tree.size(), (i + 1) as usize);
        }
    }

    #[test]
    pub fn test_drop() {
        let mut tree = SimpleTree::new();

        for i in 0..10000 {
            tree.insert(i);
            assert_eq!(tree.find(i), Some(&i));
            assert_eq!(tree.size(), (i + 1) as usize);
        }

        drop(tree);
    }
}
