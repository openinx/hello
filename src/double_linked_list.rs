use std::mem;
use std::ptr;

pub struct List<T> {
    list_head: Option<Box<Node<T>>>,
    list_tail: Rawlink<Node<T>>,
}

struct Rawlink<T> {
    p: *mut T,
}

impl<T> Copy for Rawlink<T> {}

impl<T> Clone for Rawlink<T> {
    fn clone(&self) -> Self {
        Rawlink { p: self.p }
    }
}

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    prev: Rawlink<Node<T>>,
    value: T,
}

impl<T> List<T> {
    pub fn is_empty(&self) -> bool {
        self.list_head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut node = &self.list_head;
        let mut i = 0;
        loop {
            match *node {
                Some(ref n) => {
                    i += 1;
                    node = &n.next;
                }
                None => {
                    return i;
                }
            }
        }
    }

    /// Create an empty DList
    pub fn new() -> List<T> {
        List {
            list_head: None,
            list_tail: Rawlink::none(),
        }
    }

    pub fn push_front(&mut self, elt: T) {
        self.push_front_node(Box::new(Node::new(elt)))
    }

    pub fn push_front_node(&mut self, mut new_head: Box<Node<T>>) {
        match self.list_head {
            None => {
                self.list_tail = Rawlink::some(&mut new_head);
                new_head.prev = Rawlink::none();
                self.list_head = Some(new_head);
            }
            Some(ref mut head) => {
                new_head.prev = Rawlink::none();
                head.prev = Rawlink::some(&mut new_head);
                mem::swap(head, &mut new_head);
                head.next = Some(new_head);
            }
        }
    }

    /// Provide a forward iterator
    #[inline]
    pub fn iter<'a>(&'a self) -> ListIterator<'a, T> {
        ListIterator {
            nelem: self.len(),
            head: &self.list_head,
            tail: self.list_tail,
        }
    }
}

impl<T> Node<T> {
    fn new(v: T) -> Node<T> {
        Node {
            value: v,
            next: None,
            prev: Rawlink::none(),
        }
    }
}

/// Rawlink is a type like Option<T> but for holding a raw pointer
impl<T> Rawlink<T> {
    /// Like Option::None for Rawlink
    fn none() -> Rawlink<T> {
        Rawlink { p: ptr::null_mut() }
    }

    /// Like Option::Some for Rawlink
    fn some(n: &mut T) -> Rawlink<T> {
        Rawlink { p: n as *mut T }
    }

    /// Convert the `Rawlink` into an Option value
    fn resolve_immut<'a>(&self) -> Option<&'a T> {
        unsafe { self.p.as_ref() }
    }

    /// Convert the `Rawlink` into an Option value
    fn resolve<'a>(&mut self) -> Option<&'a mut T> {
        unsafe { self.p.as_mut() }
    }

    /// Return the `Rawlink` and replace with `Rawlink::none()`
    fn take(&mut self) -> Rawlink<T> {
        mem::replace(self, Rawlink::none())
    }
}

pub struct ListIterator<'a, T: 'a> {
    head: &'a Option<Box<Node<T>>>,
    tail: Rawlink<Node<T>>,
    nelem: usize,
}

impl<'a, A> Iterator for ListIterator<'a, A> {
    type Item = &'a A;

    #[inline]
    fn next(&mut self) -> Option<&'a A> {
        if self.nelem == 0 {
            return None;
        }
        self.head.as_ref().map(|head| {
            self.nelem -= 1;
            self.head = &head.next;
            &head.value
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.nelem, Some(self.nelem))
    }
}

impl<'a, A> DoubleEndedIterator for ListIterator<'a, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a A> {
        if self.nelem == 0 {
            return None;
        }
        let tmp = self.tail.resolve_immut();
        tmp.as_ref().map(|prev| {
            self.nelem -= 1;
            self.tail = prev.prev;
            &prev.value
        })
    }
}
