use std::ptr;

pub struct List<T: Eq> {
    head: Link<T>,
    tail: Link<T>,
}

/// `*mut` means a raw pointer.
type Link<T> = *mut Node<T>;

struct Node<T: Eq> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T: Eq>(List<T>);

pub struct Iter<'a, T: Eq> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: Eq> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T>
where
    T: Eq,
{
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                // *self.tail to access the entry which raw pointer is pointing to.
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new_head = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
            }));

            if self.head.is_null() {
                self.head = new_head;
                self.tail = new_head;
            } else {
                (*new_head).next = self.head;
                self.head = new_head;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }

    pub fn find(&self, elem: T) -> bool {
        let mut cur = self.head;
        while !cur.is_null() {
            unsafe {
                if (*cur).elem == elem {
                    return true;
                }
                cur = (*cur).next;
            }
        }
        return false;
    }

    pub fn remove(&mut self, elem: T) -> bool {
        unsafe {
            if self.head.is_null() {
                return false;
            }

            if (*self.head).elem == elem {
                let next = (*self.head).next;
                self.head = next;

                if next.is_null() {
                    // Only one node where both head and tail are pointing to.
                    self.tail = next;
                }
                return true;
            }

            let mut prev = self.head;
            let mut cur = (*prev).next;
            while !cur.is_null() {
                if (*cur).elem == elem {
                    let next = (*cur).next;
                    (*prev).next = next;

                    if next.is_null() {
                        // The removing node is the one that tail is pointing to.
                        self.tail = prev;
                    }
                    return true;
                }

                prev = cur;
                cur = (*prev).next;
            }
        }

        return false;
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T: Eq> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> Iterator for IntoIter<T>
where
    T: Eq,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Eq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Eq,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push_back(6);
        list.push_back(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_remove() {
        let mut list = List::new();

        {
            (0..5).for_each(|i| list.push_back(i));
            (0..5).for_each(|i| assert_eq!(list.find(i), true));

            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&0));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), None);
        }

        {
            (0..2).for_each(|i| assert_eq!(list.remove(i), true));
            (0..2).for_each(|i| assert_eq!(list.find(i), false));
            (2..5).for_each(|i| assert_eq!(list.find(i), true));

            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&4));
        }

        {
            (2..5).for_each(|i| assert_eq!(list.remove(i), true));
            (0..5).for_each(|i| assert_eq!(list.find(i), false));

            let mut iter = list.iter();
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn test_remove_tail() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);

        assert_eq!(true, list.remove(2));
        assert_eq!(true, list.find(1));
        assert_eq!(false, list.find(2));

        list.push_back(3);
        assert_eq!(true, list.find(3));

        assert_eq!(true, list.remove(3));
        assert_eq!(true, list.find(1));
        assert_eq!(false, list.find(2));
        assert_eq!(false, list.find(3));

        assert_eq!(true, list.remove(1));
        assert_eq!(false, list.find(1));
        assert_eq!(false, list.find(2));
        assert_eq!(false, list.find(3));
    }

    #[test]
    fn test_remove_head() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);

        assert_eq!(true, list.remove(1));
        assert_eq!(false, list.find(1));
        assert_eq!(true, list.find(2));

        assert_eq!(true, list.remove(2));
        assert_eq!(false, list.find(1));
        assert_eq!(false, list.find(2));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_push_front() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_back(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert!(list.pop() == Some(1));
        list.push_back(4);
        assert!(list.pop() == Some(2));
        list.push_back(5);

        assert!(list.peek() == Some(&3));
        list.push_back(6);
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&30));
        assert!(list.pop() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop() == Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&5000));
        list.push_back(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}
