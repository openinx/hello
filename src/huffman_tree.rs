struct HuffmanTree {
    n: usize,
    weight: Vec<usize>,
    parent: Vec<usize>,
    lchild: Vec<usize>,
    rchild: Vec<usize>,
}

const NIL: usize = usize::MAX;

impl HuffmanTree {
    fn new(w: Vec<usize>) -> Self {
        if w.len() == 0 {
            HuffmanTree {
                n: 0,
                weight: Vec::new(),
                parent: Vec::new(),
                lchild: Vec::new(),
                rchild: Vec::new(),
            }
        } else {
            let n = w.len();

            let mut weight = w.to_vec();
            for _ in n..(2 * n - 1) {
                // Fill the weight[n], weight[n+1], .., weight[2*n-1] with NIL.
                weight.push(NIL);
            }

            HuffmanTree {
                n,
                weight,
                parent: vec![NIL; 2 * n - 1],
                lchild: vec![NIL; 2 * n - 1],
                rchild: vec![NIL; 2 * n - 1],
            }
        }
    }

    fn build(&mut self) {
        for cur in self.n..(2 * self.n - 1) {
            let mut left = NIL;
            let mut left_weight = NIL;
            for k in 0..cur {
                if self.parent[k] == NIL && (left_weight == NIL || self.weight[k] < left_weight) {
                    left = k;
                    left_weight = self.weight[k];
                }
            }

            let mut right = NIL;
            let mut right_weight = NIL;
            for k in 0..cur {
                if k != left
                    && self.parent[k] == NIL
                    && (right_weight == NIL || self.weight[k] < right_weight)
                {
                    right = k;
                    right_weight = self.weight[k];
                }
            }

            assert!(left != NIL);
            assert!(right != NIL);

            self.parent[left] = cur;
            self.parent[right] = cur;

            self.lchild[cur] = left;
            self.rchild[cur] = right;
            self.weight[cur] = self.weight[left] + self.weight[right];
        }
    }

    fn total_weight(&self) -> usize {
        let mut total = 0;
        for i in 0..self.n {
            let mut step = 0;

            // Tracing back to the parent from current node i.
            let mut index = i;
            while self.parent[index] != NIL {
                step += 1;
                index = self.parent[index];
            }

            total += step * self.weight[i];
        }
        return total;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let mut tree = HuffmanTree::new(vec![5, 29, 7, 8, 14, 23, 3, 11]);
        tree.build();
        assert_eq!(tree.total_weight(), 271);

        tree = HuffmanTree::new(vec![7, 5, 2, 4]);
        tree.build();
        assert_eq!(tree.total_weight(), 35);
    }
}
