use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Node {
            elem: elem,
            next: self.head.take(),
        };
        self.head = Some(Rc::new(new_node));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next.clone();
            Rc::try_unwrap(node).ok().unwrap().elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

pub fn merge_lists(a: &mut List<i32>, b: &mut List<i32>) -> List<i32> {
    let mut res = List::new();

    while !a.is_empty() && !b.is_empty() {
        let a_elem = a.peek().unwrap();
        let b_elem = b.peek().unwrap();

        if a_elem < b_elem {
            res.push_front(a.pop_front().unwrap());
        } else {
            res.push_front(b.pop_front().unwrap());
        }
    }

    while !a.is_empty() {
        res.push_front(a.pop_front().unwrap());
    }

    while !b.is_empty() {
        res.push_front(b.pop_front().unwrap());
    }

    let mut reversed = List::new();
    while !res.is_empty() {
        reversed.push_front(res.pop_front().unwrap());
    }

    // TODO free the temporary list res.
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);

        assert_eq!(Some(&2), list.peek());
        assert_eq!(Some(2), list.pop_front());

        assert_eq!(Some(&1), list.peek());
        assert_eq!(Some(1), list.pop_front());
    }

    #[test]
    pub fn test_merge_sort() {
        let mut a = List::new();
        let mut b = List::new();

        a.push_front(8);
        a.push_front(5);
        a.push_front(3);

        b.push_front(11);
        b.push_front(3);

        let mut res = merge_lists(&mut a, &mut b);

        assert_eq!(Some(3), res.pop_front());
        assert_eq!(Some(3), res.pop_front());
        assert_eq!(Some(5), res.pop_front());
        assert_eq!(Some(8), res.pop_front());
        assert_eq!(Some(11), res.pop_front());
        assert_eq!(true, res.is_empty());
    }
}
