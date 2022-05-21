use std::rc::Rc;

pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

pub struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: Option::None }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn prepend(&mut self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&mut self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        assert_eq!(Some(&3), list.head());

        list = list.tail();
        assert_eq!(Some(&2), list.head());

        list = list.tail();
        assert_eq!(Some(&1), list.head());

        list = list.tail();
        assert_eq!(None, list.head());

        list = list.tail();
        assert_eq!(None, list.head());
    }

    #[test]
    pub fn iter_i32() {
        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        let mut it = list.iter();
        assert_eq!(Some(&3), it.next());
        assert_eq!(Some(&2), it.next());
        assert_eq!(Some(&1), it.next());
        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    pub fn iter_string() {
        let mut list = List::new();

        list = list.prepend("Hello");
        list = list.prepend("world");

        let mut it = list.iter();
        assert_eq!(Some(&"world"), it.next());
        assert_eq!(Some(&"Hello"), it.next());
        assert_eq!(None, it.next());
    }
}
