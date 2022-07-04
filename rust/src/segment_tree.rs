struct SegmentTree {
    n: u64,
    nodes: Vec<Node>,
}

struct Node {
    // Logical pointer to the left child and right child.
    lc: usize,
    rc: usize,

    // The interval [left, right) for the current node.
    left: u64,
    right: u64,

    // The value for this node.
    value: u64,
}

const NIL: usize = usize::MAX;

impl Node {
    fn new(lc: usize, rc: usize, left: u64, right: u64, value: u64) -> Self {
        Node {
            lc,
            rc,
            left,
            right,
            value,
        }
    }
}

impl SegmentTree {
    fn new(n: u64) -> Self {
        assert!(n > 0);
        SegmentTree {
            n,
            nodes: Vec::new(),
        }
    }

    fn init(&mut self) {
        self._init(0, self.n);
    }

    fn _init(&mut self, left: u64, right: u64) -> usize {
        self.nodes.push(Node::new(NIL, NIL, left, right, 0));
        let idx = self.nodes.len() - 1;

        if left + 1 >= right {
            return idx;
        }

        let middle = (left + right) >> 1;

        // Initialize the left children.
        if left < middle {
            self.nodes[idx].lc = self._init(left, middle);
        }

        // Initialize the right children.
        if middle < right {
            self.nodes[idx].rc = self._init(middle, right);
        }

        return idx;
    }

    fn update(&mut self, left: u64, right: u64, inc: u64) {
        self._update(0, left, right, inc);
    }

    fn _update(&mut self, idx: usize, left: u64, right: u64, inc: u64) {
        assert!(left < right);

        if left <= self.nodes[idx].left && self.nodes[idx].right <= right {
            self.nodes[idx].value += inc;
            return;
        }

        let middle = (self.nodes[idx].left + self.nodes[idx].right) >> 1;
        if left < middle && self.nodes[idx].lc != NIL {
            self._update(self.nodes[idx].lc, left, right, inc);
        }

        if middle < right && self.nodes[idx].rc != NIL {
            self._update(self.nodes[idx].rc, left, right, inc);
        }
    }

    fn query(&self, x: u64) -> u64 {
        return self._query(0, x);
    }

    fn _query(&self, idx: usize, x: u64) -> u64 {
        assert!(idx < self.nodes.len());

        let mut sum = self.nodes[idx].value;
        let middle = (self.nodes[idx].left + self.nodes[idx].right) >> 1;

        if x < middle && self.nodes[idx].lc != NIL {
            sum += self._query(self.nodes[idx].lc, x);
        }

        if middle <= x && self.nodes[idx].rc != NIL {
            sum += self._query(self.nodes[idx].rc, x)
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basic() {
        let mut tree = SegmentTree::new(5);
        tree.init();

        tree.update(0, 1, 1);

        assert_eq!(1, tree.query(0));
        assert_eq!(0, tree.query(1));
        assert_eq!(0, tree.query(2));
        assert_eq!(0, tree.query(3));
        assert_eq!(0, tree.query(4));

        tree.update(0, 5, 1);
        assert_eq!(2, tree.query(0));
        assert_eq!(1, tree.query(1));
        assert_eq!(1, tree.query(2));
        assert_eq!(1, tree.query(3));
        assert_eq!(1, tree.query(4));

        tree.update(4, 5, 1);
        assert_eq!(2, tree.query(0));
        assert_eq!(1, tree.query(1));
        assert_eq!(1, tree.query(2));
        assert_eq!(1, tree.query(3));
        assert_eq!(2, tree.query(4));

        tree.update(0, 3, 1);
        assert_eq!(3, tree.query(0));
        assert_eq!(2, tree.query(1));
        assert_eq!(2, tree.query(2));
        assert_eq!(1, tree.query(3));
        assert_eq!(2, tree.query(4));

        tree.update(2, 5, 1);
        assert_eq!(3, tree.query(0));
        assert_eq!(2, tree.query(1));
        assert_eq!(3, tree.query(2));
        assert_eq!(2, tree.query(3));
        assert_eq!(3, tree.query(4));

        tree.update(1, 5, 2);
        assert_eq!(3, tree.query(0));
        assert_eq!(4, tree.query(1));
        assert_eq!(5, tree.query(2));
        assert_eq!(4, tree.query(3));
        assert_eq!(5, tree.query(4));
    }
}
