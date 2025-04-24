use std::collections::VecDeque;
use std::mem::swap;

struct Tree {
    tree: Vec<Vec<usize>>,
    parent: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl Tree {
    fn new(size: usize, tree: Vec<Vec<usize>>) -> Self {
        Self { 
            tree, 
            parent: vec![vec![0; 17]; size + 1], 
            depth: vec![0; size + 1],
        }
    }

    fn init_parent(&mut self, root: usize) {
        let mut q = VecDeque::new();
        self.depth[root] = 1;
        self.parent[root][0] = 0;
        q.push_back(root);

        while let Some(node) = q.pop_front() {
            for &child in self.tree[node].iter() {
                if child != self.parent[node][0] {
                    self.depth[child] = self.depth[node] + 1;
                    self.parent[child][0] = node;
                    q.push_back(child);
                }
            }
        }
    }

    fn fill_parent(&mut self) {
        for j in 1..17 {
            for i in 1..self.tree.len() {
                self.parent[i][j] = self.parent[self.parent[i][j - 1]][j - 1];
            }
        }
    }

    fn query(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            swap(&mut a, &mut b);
        }

        let diff = self.depth[a] - self.depth[b];

        for i in 0..17 {
            if diff & (1 << i) != 0 {
                a = self.parent[a][i];
            }
        }

        if a != b {
            for i in (0..17).rev() {
                if self.parent[a][i] != self.parent[b][i] {
                    a = self.parent[a][i];
                    b = self.parent[b][i];
                }
            }
            a = self.parent[a][0];
        }

        a
    }
}