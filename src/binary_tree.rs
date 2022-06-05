pub struct Tree<T> {
    size: usize,
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    l: Link<T>,
    r: Link<T>,
}

impl<T> Tree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Tree {
            size: 0,
            root: None,
        }
    }

    fn get_mut(&mut self, elem: T) -> Option<&mut Box<Node<T>>> {
        let mut cur = &mut self.root;
        loop {
            match cur {
                None => return cur.as_mut(),
                Some(node) => {
                    if elem < node.elem {
                        cur = &mut node.l;
                    } else if elem > node.elem {
                        cur = &mut node.r;
                    } else {
                        return Some(node);
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, elem: T) {}

    pub fn remove(&mut self, elem: T) -> Option<T> {
        todo!()
    }

    pub fn find(&self, elem: T) -> Option<&T> {
        todo!()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {}
}
