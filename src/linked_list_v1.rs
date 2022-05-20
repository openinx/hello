/**
 * Linked list v1.
 * Referece to https://rust-unofficial.github.io/too-many-lists/first-final.html
 */
use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Rust cannot assign referece to borrowed &self.head. So here we use
        // mem.replace to get a reference copy.
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    pub fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        assert_eq!(list.pop().unwrap(), 2);
        assert_eq!(list.pop().unwrap(), 1);
        assert_eq!(list.pop(), None);

        for i in 0..=9 {
            list.push(i);
        }
        for i in 0..=9 {
            assert_eq!(list.pop().unwrap(), 9 - i);
        }
        assert_eq!(list.pop(), None);
    }
}
