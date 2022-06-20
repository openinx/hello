pub struct Graph {
    pub node_size: usize,
    pub links: Vec<Link>,
}

pub struct Node {
    pub id: usize,
    pub next: Link,
}

pub type Link = Option<Box<Node>>;

impl Graph {
    pub fn new(node_size: usize) -> Self {
        Graph {
            node_size,
            links: (0..node_size).map(|_| None).collect(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.links.len());
        assert!(to < self.links.len());

        match self.links[from].take() {
            None => {
                self.links[from] = Some(Box::new(Node { id: to, next: None }));
            }
            Some(node) => {
                self.links[from] = Some(Box::new(Node {
                    id: to,
                    next: Some(node),
                }));
            }
        }
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        for i in 0..self.links.len() {
            let cur = &mut self.links[i];
            while let Some(node) = cur.take() {
                *cur = node.next;
            }
        }
    }
}

fn _dfs(g: &Graph, id: usize, visit: &mut Vec<bool>, result: &mut Vec<usize>) {
    assert_eq!(visit[id], true);

    let mut cur = &g.links[id];
    while let Some(node) = cur {
        if !visit[node.id] {
            visit[node.id] = true;
            result.push(node.id);

            _dfs(g, node.id, visit, result);
        }
        cur = &node.next;
    }
}

fn dfs(g: &Graph) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut visit = vec![false; g.node_size];
    for id in 0..g.node_size {
        if !visit[id] {
            visit[id] = true;
            result.push(id);

            _dfs(g, id, &mut visit, &mut result);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{dfs, Graph};

    #[test]
    pub fn basics() {
        let mut g = Graph::new(10);

        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(1, 4);
        g.add_edge(2, 3);
        g.add_edge(3, 4);
        g.add_edge(4, 0);

        assert_eq!(dfs(&g), vec![0, 1, 4, 3, 2, 5, 6, 7, 8, 9]);
    }
}
