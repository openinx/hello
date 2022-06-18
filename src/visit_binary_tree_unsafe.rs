use std::ptr;

struct Tree<T: Eq> {
    root: Link<T>,
}

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }
}

type Link<T> = *mut Node<T>;

trait TreeLink<T> {
    fn find_mut(self, elem: &T) -> Link<T>;

    fn prev_visit<'a>(&'a self, v: &mut Vec<&'a T>);

    fn inorder_visit<'a>(&'a self, v: &mut Vec<&'a T>);

    fn post_visit<'a>(&'a self, v: &mut Vec<&'a T>);

    fn drop(self);
}

impl<T> TreeLink<T> for Link<T>
where
    T: Eq,
{
    fn find_mut(self, elem: &T) -> Link<T> {
        if self.is_null() {
            return ptr::null_mut();
        }

        unsafe {
            if (*self).elem == *elem {
                return self;
            }

            let res = (*self).left.find_mut(elem);
            if !res.is_null() {
                return res;
            }

            (*self).right.find_mut(elem)
        }
    }

    fn prev_visit<'a>(&'a self, v: &mut Vec<&'a T>) {
        if !self.is_null() {
            unsafe {
                let cur = &(*(*self));
                v.push(&cur.elem);

                cur.left.prev_visit(v);
                cur.right.prev_visit(v);
            }
        }
    }

    fn post_visit<'a>(&'a self, v: &mut Vec<&'a T>) {
        if !self.is_null() {
            unsafe {
                let cur = &(*(*self));
                cur.left.post_visit(v);
                cur.right.post_visit(v);

                v.push(&cur.elem);
            }
        }
    }

    fn inorder_visit<'a>(&'a self, v: &mut Vec<&'a T>) {
        if !self.is_null() {
            unsafe {
                let cur = &(*(*self));
                cur.left.inorder_visit(v);
                v.push(&cur.elem);
                cur.right.inorder_visit(v);
            }
        }
    }

    fn drop(self) {
        if !self.is_null() {
            unsafe {
                (*self).left.drop();
                (*self).left = ptr::null_mut();
                (*self).right.drop();
                (*self).right = ptr::null_mut();
            }
        }
    }
}

impl<T> Tree<T>
where
    T: Eq,
{
    pub fn new() -> Self {
        Tree {
            root: ptr::null_mut(),
        }
    }

    pub fn init_root(&mut self, elem: T) {
        self.root = Box::into_raw(Box::new(Node::new(elem)));
    }

    pub fn add_left_child(&mut self, parent: T, child: T) {
        unsafe {
            let p = self.root.find_mut(&parent);
            if !p.is_null() {
                (*p).left = Box::into_raw(Box::new(Node::new(child)));
            }
        }
    }

    pub fn add_right_child(&mut self, parent: T, child: T) {
        unsafe {
            let p = self.root.find_mut(&parent);
            if !p.is_null() {
                (*p).right = Box::into_raw(Box::new(Node::new(child)));
            }
        }
    }

    pub fn pre_visit(&self) -> Vec<&T> {
        let mut order = Vec::new();
        self.root.prev_visit(&mut order);
        return order;
    }

    pub fn inorder_visit(&self) -> Vec<&T> {
        let mut order = Vec::new();
        self.root.inorder_visit(&mut order);
        return order;
    }

    pub fn post_visit(&self) -> Vec<&T> {
        let mut order = Vec::new();
        self.root.post_visit(&mut order);
        return order;
    }
}

impl<T> Drop for Tree<T>
where
    T: Eq,
{
    fn drop(&mut self) {
        self.root.drop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let mut tree = Tree::new();
        tree.init_root(10);

        tree.add_left_child(10, 5);
        tree.add_right_child(10, 20);
        tree.add_left_child(5, 3);
        tree.add_right_child(5, 8);
        tree.add_left_child(20, 13);
        tree.add_left_child(13, 12);
        tree.add_left_child(12, 11);

        assert_eq!(tree.pre_visit(), vec![&10, &5, &3, &8, &20, &13, &12, &11]);
        assert_eq!(
            tree.inorder_visit(),
            vec![&3, &5, &8, &10, &11, &12, &13, &20]
        );
        assert_eq!(tree.post_visit(), vec![&3, &8, &5, &11, &12, &13, &20, &10]);
    }

    #[test]
    pub fn empty_tree() {
        let tree: Tree<i32> = Tree::new();
        assert_eq!(tree.pre_visit().is_empty(), true);
        assert_eq!(tree.inorder_visit().is_empty(), true);
        assert_eq!(tree.post_visit().is_empty(), true);
    }

    #[test]
    pub fn single_node() {
        let mut tree = Tree::new();
        tree.init_root(1);

        assert_eq!(tree.pre_visit(), vec![&1]);
        assert_eq!(tree.inorder_visit(), vec![&1]);
        assert_eq!(tree.post_visit(), vec![&1]);
    }

    #[test]
    pub fn two_nodes() {
        {
            let mut tree = Tree::new();
            tree.init_root(3);
            tree.add_left_child(3, 1);

            assert_eq!(tree.pre_visit(), vec![&3, &1]);
            assert_eq!(tree.inorder_visit(), vec![&1, &3]);
            assert_eq!(tree.post_visit(), vec![&1, &3]);
        }

        {
            let mut tree = Tree::new();
            tree.init_root(3);
            tree.add_right_child(3, 4);

            assert_eq!(tree.pre_visit(), vec![&3, &4]);
            assert_eq!(tree.inorder_visit(), vec![&3, &4]);
            assert_eq!(tree.post_visit(), vec![&4, &3]);
        }
    }

    #[test]
    pub fn singe_list() {
        {
            let mut tree = Tree::new();
            tree.init_root(5);
            for i in (0..5).rev() {
                tree.add_left_child(i + 1, i);
            }

            assert_eq!(tree.pre_visit(), vec![&5, &4, &3, &2, &1, &0]);
            assert_eq!(tree.inorder_visit(), vec![&0, &1, &2, &3, &4, &5]);
            assert_eq!(tree.post_visit(), vec![&0, &1, &2, &3, &4, &5]);
        }

        {
            let mut tree = Tree::new();
            tree.init_root(5);
            for i in 6..=10 {
                tree.add_right_child(i - 1, i);
            }

            assert_eq!(tree.pre_visit(), vec![&5, &6, &7, &8, &9, &10]);
            assert_eq!(tree.inorder_visit(), vec![&5, &6, &7, &8, &9, &10]);
            assert_eq!(tree.post_visit(), vec![&10, &9, &8, &7, &6, &5]);
        }
    }
}
