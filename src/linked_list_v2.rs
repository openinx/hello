
pub struct List<T>{
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    elem: T,
    next: Link<T>
}

impl<T> List<T> {

    pub fn new() -> Self {
        List {head: Option::None} 
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem, next: self.head.take()
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests{
    use super::List;

    #[test]
    pub fn basics(){
        let mut list = List::new();

        assert_eq!(list.is_empty(), true);
        list.push(1);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.is_empty(), true);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.peek(), Option::None);
    }
}