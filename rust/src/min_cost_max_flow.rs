#[derive(Default, Clone, Copy)]
struct Edge {
    to: usize,
    capacity: i32,
    cost: i32,
    rev: usize,
}

struct FlowGraph {
    g: Vec<Vec<Edge>>,
    dist: Vec<i32>,
    from: Vec<usize>,
    fidx: Vec<usize>,
    used: Vec<bool>,
    s: usize,
    t: usize,
}

impl FlowGraph {
    fn new(n: usize, s: usize, t: usize) -> Self {
        Self {
            g: vec![Vec::new(); n + 1],
            dist: vec![0; n + 1],
            from: vec![0; n + 1],
            fidx: vec![0; n + 1],
            used: vec![false; n + 1],
            s, t
        }
    }

    fn connect(&mut self, u: usize, v: usize, capacity: i32, cost: i32) {
        let u_rev = self.g[v].len();
        let v_rev = self.g[u].len();

        self.g[u].push(Edge { to: v, capacity, cost, rev: u_rev});
        self.g[v].push(Edge { to: u, capacity: 0, cost: -cost, rev: v_rev});
    }

    fn spfa(&mut self) -> bool {
        self.dist.fill(i32::MAX);
        let mut q = VecDeque::new();
        self.dist[self.s] = 0;
        self.used[self.s] = true;
        q.push_back(self.s);

        while let Some(cur) = q.pop_front() {
            self.used[cur] = false;

            for (idx, nxt_edge) in self.g[cur].iter_mut().enumerate() {
                if nxt_edge.capacity == 0 {
                    continue;
                }

                let nxt = nxt_edge.to;
                if self.dist[nxt] > self.dist[cur] + nxt_edge.cost {
                    self.from[nxt] = cur;
                    self.fidx[nxt] = idx;
                    self.dist[nxt] = self.dist[cur] + nxt_edge.cost;
                    if !self.used[nxt] {
                        q.push_back(nxt);
                        self.used[nxt] = true;
                    }
                }
            }
        }
        self.dist[self.t] != i32::MAX
    }


    fn backtrack(&mut self, cur: usize, flow: i32) -> (i32, i32) {
        if cur == self.s {
            return (flow, 0);
        }

        let from = self.from[cur];
        let cost = self.g[from][self.fidx[cur]].cost;
        let rev_idx = self.g[from][self.fidx[cur]].rev;

        let res = self.backtrack(from, flow.min(self.g[from][self.fidx[cur]].capacity));

        self.g[from][self.fidx[cur]].capacity -= res.0;
        self.g[cur][rev_idx].capacity += res.0;

        (res.0, res.1 + cost)
    }

    fn min_cost_max_flow(&mut self) -> (i32, i32) {
        let mut max_flow = 0;
        let mut min_cost = 0;

        while self.spfa() {
            let (flow, cost) = self.backtrack(self.t, i32::MAX);
            max_flow += flow;
            min_cost += flow * cost;
        }

        (max_flow, min_cost)
    }
}
