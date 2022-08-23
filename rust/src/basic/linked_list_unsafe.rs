use std::ptr;

pub struct List<T: Eq> {
    head: Link<T>,
    tail: Link<T>,
}

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

    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
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
                self.head = (*self.head).next;
                return true;
            }

            let mut prev = self.head;
            let mut curr = (*self.head).next;
            while !curr.is_null() {
                if (*curr).elem == elem {
                    (*prev).next = (*curr).next;
                    return true;
                }

                prev = curr;
                curr = (*prev).next;
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
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_remove() {
        let mut list = List::new();

        {
            (0..5).for_each(|i| list.push(i));
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
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop() == Some(1));
        list.push(4);
        assert!(list.pop() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
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
        list.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}
