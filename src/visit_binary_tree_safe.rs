pub struct Tree<T: Eq> {
    root: Link<T>,
}

pub struct Node<T: Eq> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

trait TreeLink<T: Eq> {
    fn find_mut(&mut self, elem: &T) -> Option<&mut Link<T>>;

    fn prev_visit<'a>(&'a self, order: &mut Vec<&'a T>);

    fn inorder_visit<'a>(&'a self, order: &mut Vec<&'a T>);

    fn post_visit<'a>(&'a self, order: &mut Vec<&'a T>);

    fn drop(&mut self);
}

impl<T> TreeLink<T> for Link<T>
where
    T: Eq,
{
    fn find_mut(&mut self, elem: &T) -> Option<&mut Link<T>> {
        match self {
            None => return None,
            Some(node) if node.elem == *elem => {
                return Some(self);
            }
            Some(node) => {
                let res = node.left.find_mut(elem);
                if res.is_some() {
                    return res;
                }

                node.right.find_mut(elem)
            }
        }
    }

    fn prev_visit<'a>(&'a self, order: &mut Vec<&'a T>) {
        if let Some(node) = self {
            order.push(&node.elem);
            node.left.prev_visit(order);
            node.right.prev_visit(order);
        }
    }

    fn inorder_visit<'a>(&'a self, order: &mut Vec<&'a T>) {
        if let Some(node) = self {
            node.left.inorder_visit(order);
            order.push(&node.elem);
            node.right.inorder_visit(order);
        }
    }

    fn post_visit<'a>(&'a self, order: &mut Vec<&'a T>) {
        if let Some(node) = self {
            node.left.post_visit(order);
            node.right.post_visit(order);
            order.push(&node.elem);
        }
    }

    fn drop(&mut self) {
        self.as_mut().take().map(|node| {
            node.left.drop();
            node.right.drop();
        });
    }
}

impl<T> Tree<T>
where
    T: Eq,
{
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn init_root(&mut self, elem: T) {
        self.root = Some(Box::new(Node {
            elem,
            left: None,
            right: None,
        }));
    }

    pub fn add_left_child(&mut self, parent: T, child: T) {
        self.root.find_mut(&parent).map(|parent_node| {
            parent_node.as_mut().unwrap().left.replace(Box::new(Node {
                elem: child,
                left: None,
                right: None,
            }));
        });
    }

    pub fn add_right_child(&mut self, parent: T, child: T) {
        self.root.find_mut(&parent).map(|parent_node| {
            parent_node.as_mut().unwrap().right.replace(Box::new(Node {
                elem: child,
                left: None,
                right: None,
            }));
        });
    }

    pub fn pre_visit<'a>(&'a self) -> Vec<&'a T> {
        let mut order: Vec<&T> = Vec::new();
        self.root.prev_visit(&mut order);
        return order;
    }

    pub fn inorder_visit<'a>(&'a self) -> Vec<&'a T> {
        let mut order: Vec<&T> = Vec::new();
        self.root.inorder_visit(&mut order);
        return order;
    }

    pub fn post_visit<'a>(&'a self) -> Vec<&'a T> {
        let mut order: Vec<&T> = Vec::new();
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
