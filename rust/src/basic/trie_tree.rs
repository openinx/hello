struct TrieTree {
    root: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    children: Vec<Link>,
    count: usize,
}

impl Node {
    fn new() -> Link {
        Some(Box::new(Node {
            children: (0..26).map(|_| None).collect(),
            count: 0,
        }))
    }

    fn child(&self, i: u8) -> &Link {
        let idx = i as usize - 'a' as usize;
        assert!(idx < self.children.len());
        &self.children[idx]
    }

    fn child_mut(&mut self, i: u8) -> &mut Link {
        let idx = i as usize - 'a' as usize;
        assert!(idx < self.children.len());
        &mut self.children[idx]
    }

    fn inc(&mut self) {
        self.count += 1;
    }
}

impl TrieTree {
    fn new() -> Self {
        TrieTree { root: Node::new() }
    }

    fn add(&mut self, s: &str) {
        let mut ptr = &mut self.root;
        let bytes = s.as_bytes();

        for i in 0..bytes.len() {
            ptr = ptr.as_mut().unwrap().child_mut(bytes[i]);
            if ptr.is_none() {
                *ptr = Node::new();
            }
        }

        ptr.as_mut().unwrap().inc();
    }

    fn find(&self, s: &str) -> bool {
        let mut ptr = &self.root;
        let bytes = s.as_bytes();

        for i in 0..bytes.len() {
            ptr = ptr.as_ref().unwrap().child(bytes[i]);
            if ptr.is_none() {
                return false;
            }
        }

        ptr.as_ref().unwrap().count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::TrieTree;

    #[test]
    pub fn basics() {
        let mut tree = TrieTree::new();
        tree.add("");
        assert_eq!(tree.find(""), true);

        tree.add("a");
        assert_eq!(tree.find("a"), true);
        assert_eq!(tree.find(""), true);

        tree.add("aa");
        assert_eq!(tree.find(""), true);
        assert_eq!(tree.find("a"), true);
        assert_eq!(tree.find("aa"), true);
        assert_eq!(tree.find("aaa"), false);

        tree.add("aaa");
        assert_eq!(tree.find(""), true);
        assert_eq!(tree.find("a"), true);
        assert_eq!(tree.find("aa"), true);
        assert_eq!(tree.find("aaa"), true);

        assert_eq!(tree.find("aaab"), false);
    }
}
