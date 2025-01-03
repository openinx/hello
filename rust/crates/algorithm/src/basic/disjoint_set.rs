use std::collections::HashMap;

struct DisjointSet {
    len: usize,
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(len: usize) -> Self {
        DisjointSet {
            len,
            parent: (0..len).collect(),
            rank: vec![1; len],
        }
    }

    // Find the root of the node `x`.
    fn find(&mut self, x: usize) -> usize {
        assert!(x < self.len);
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        } else {
            x
        }
    }

    fn merge(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx != ry {
            if self.rank[rx] > self.rank[ry] {
                self.rank[rx] += self.rank[ry];
                self.parent[ry] = rx;
            } else {
                self.rank[ry] += self.rank[rx];
                self.parent[rx] = ry;
            }
        }
    }

    fn subset(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..self.len {
            let root = self.find(i);
            map.entry(root).or_insert(Vec::new()).push(i);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn basics() {
        let mut set = DisjointSet::new(10);
        assert_eq!(
            (0..10).map(|x| (x, vec![x])).collect::<HashMap<_, _>>(),
            set.subset()
        );

        set.merge(0, 1);
        set.merge(2, 3);
        set.merge(4, 5);
        set.merge(6, 7);
        set.merge(8, 9);
        assert_eq!(5, set.subset().len());

        set.merge(0, 2);
        set.merge(4, 6);
        assert_eq!(3, set.subset().len());

        set.merge(2, 1);
        assert_eq!(3, set.subset().len());

        set.merge(0, 4);
        assert_eq!(2, set.subset().len());

        set.merge(0, 9);
        assert_eq!(1, set.subset().len());

        set.merge(1, 8);
        assert_eq!(1, set.subset().len());
    }
}
