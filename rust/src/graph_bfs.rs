use crate::graph_dfs::Graph;

fn bfs(g: &Graph) -> Vec<usize> {
    let mut queue = vec![0 as usize; g.node_size];

    let mut result: Vec<usize> = Vec::new();
    let mut visit = vec![false; g.node_size];
    for id in 0..g.node_size {
        if !visit[id] {
            visit[id] = true;
            result.push(id);

            let mut front = 0;
            let mut tail = 0;

            // Enqueue the node id.
            queue[tail] = id;
            tail += 1;

            while front != tail {
                let new_id = queue[front];
                front += 1;

                let mut cur = &g.links[new_id];
                while let Some(node) = cur {
                    if !visit[node.id] {
                        visit[node.id] = true;
                        result.push(node.id);

                        // Enqueue the linked node.id.
                        queue[tail] = node.id;
                        tail += 1;
                    }

                    cur = &node.next;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(bfs(&g), vec![0, 1, 4, 3, 2, 5, 6, 7, 8, 9]);
    }
}
