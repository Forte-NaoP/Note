use std::cmp::min;
use std::vec;

struct Param {
    _label: usize,
    label: Vec<usize>,
    finished: Vec<usize>,
    stack: Vec<usize>,
    scc_set: Vec<Vec<usize>>,
    scc_top: Vec<usize>
}

impl Param {
    fn new(n: usize) -> Self {
        Param{
            _label: 0, 
            label: vec![0; n + 1], 
            finished: vec![0; n + 1], 
            stack: vec![],
            scc_set: vec![],
            scc_top: vec![]
        }
    }

    fn next_label(&mut self, u: usize) -> usize {
        self._label += 1;
        self.label[u] = self._label;
        self.stack.push(u);
        self._label
    }
}

fn tarzan(g: &Vec<Vec<usize>>, n: usize) -> i32 {
    let mut p = Param::new(n);

    fn _tarzan(u: usize, p: &mut Param, g: &Vec<Vec<usize>>) -> usize {
        let mut parent = p.next_label(u);

        for &v in g[u].iter() {
            if p.label[v] == 0 {
                parent = min(parent, _tarzan(v, p, g));
            } else if p.finished[v] == 0 {
                parent = min(parent, p.label[v]);
            }
        }

        if parent == p.label[u] {
            let mut scc = vec![];
            while let Some(top) = p.stack.pop() {
                p.finished[top] = u;
                scc.push(top);
                if top == u {
                    p.scc_set.push(scc);
                    p.scc_top.push(top);
                    break;
                }
            }
        }
        parent
    }
    
    for i in 1..=n {
        if p.label[i] == 0 {
            _tarzan(i, &mut p, g);
        }
    }

    let mut indegree = vec![0; n + 1];
    for u in 1..=n {
        for &v in g[u].iter() {
            if p.finished[u] != p.finished[v] {
                indegree[p.finished[v]] += 1;
            }
        }
    }

    let mut ans = 0;
    for &i in p.scc_top.iter() {
        if indegree[i] == 0 {
            ans += 1;
        }
    }

    ans
}
