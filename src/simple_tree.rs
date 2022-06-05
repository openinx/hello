pub struct SimpleTree<T: Ord> {
    size: usize,
    root: Option<Box<Node<T>>>,
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

    pub fn size(&self) -> usize {
        self.size
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

        tree.insert(2);
        assert_eq!(tree.find(1), Some(&1));
        assert_eq!(tree.find(2), Some(&2));
        assert_eq!(tree.find(3), None);
        assert_eq!(tree.size(), 2);
        assert_eq!(tree.max(), Some(&2));
        assert_eq!(tree.min(), Some(&1));

        tree.insert(3);
        assert_eq!(tree.find(1), Some(&1));
        assert_eq!(tree.find(2), Some(&2));
        assert_eq!(tree.find(3), Some(&3));
        assert_eq!(tree.size(), 3);
        assert_eq!(tree.max(), Some(&3));
        assert_eq!(tree.min(), Some(&1));
    }

    #[test]
    pub fn test_many_i64() {
        let mut tree = SimpleTree::new();

        for i in 0..10000 {
            tree.insert(i);
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
