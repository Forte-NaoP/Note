use std::{i32, usize, vec};

struct Tree {
    root: usize,
    node: Vec<Vec<usize>>,
}

impl Tree {
    fn new(root: usize, size: usize) -> Self {
        Self {
            root, node: vec![vec![]; size + 1]
        }
    }

    fn append(&mut self, u: usize, v: usize) {
        self.node[u].push(v);
    }

    fn euler_tour(&self) -> Vec<DFSNode> {
        let mut dfs_tree = vec![DFSNode {id: 0, _in: 0, _out: 0}; self.node.len()];
        let mut dfs_id = 0;
        let mut stack = vec![(self.root, 0, 0, true)];
        while let Some((cur, parent, child_idx, enter)) = stack.pop() {
            if enter {
                dfs_tree[cur].id = cur;
                dfs_tree[cur]._in = dfs_id;
                dfs_id += 1;

                stack.push((cur, parent, child_idx, false));

                for &child in self.node[cur].iter() {
                    if child != parent {
                        stack.push((child, cur, 0, true));
                    }
                }
            } else {
                dfs_tree[cur]._out = dfs_id;
            }
        }
        dfs_tree
    }
}

#[derive(Clone, Copy)]
struct DFSNode {
    id: usize,
    _in: usize,
    _out: usize,
}