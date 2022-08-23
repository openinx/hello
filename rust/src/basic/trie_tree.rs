struct TrieTree {
    empty_str_count: usize,
    root: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    children: Vec<Link>,
    count: usize,
}

impl Node {
    fn new() -> Self {
        Node {
            children: (0..26).map(|_| None).collect(),
            count: 0,
        }
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

fn add(mut ptr: &mut Link, s: &[u8]) {
    for i in 0..s.len() {
        if ptr.is_none() {
            *ptr = Some(Box::new(Node::new()));
        }

        if let Some(n) = ptr {
            if i == s.len() - 1 {
                n.inc();
            }

            // Goto the child.
            ptr = n.child_mut(s[i]);
        }
    }
}

fn find(mut ptr: &Link, s: &[u8]) -> bool {
    for i in 0..s.len() {
        match ptr {
            None => return false,
            Some(n) => {
                if i == s.len() - 1 {
                    return n.count > 0;
                }

                ptr = n.child(s[i]);
            }
        }
    }

    return false;
}

impl TrieTree {
    fn new() -> Self {
        TrieTree {
            empty_str_count: 0,
            root: None,
        }
    }

    fn add(&mut self, s: &str) {
        if s.len() == 0 {
            self.empty_str_count += 1;
        } else {
            add(&mut self.root, s.as_bytes());
        }
    }

    fn find(&self, s: &str) -> bool {
        if s.len() == 0 {
            return self.empty_str_count > 0;
        } else {
            return find(&self.root, s.as_bytes());
        }
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
