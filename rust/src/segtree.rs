use std::vec;

struct SegTree {
    t: Vec<i64>,
}

impl SegTree {
    fn new(mut n: usize) -> Self {
        let mut b = 1;
        while n > 0 {
            b <<= 1;
            n >>= 1;
        }
        SegTree { t: vec![0; b << 1] }
    }

    fn update(&mut self, idx: usize, s: usize, e: usize, node: usize) {
        if (idx < s) || (e < idx) {
            return;
        }
        if s == e {
            self.t[node] = 1;
            return
        }
        let mid = (s + e) / 2;
        self.update(idx, s, mid, node * 2);
        self.update(idx, mid + 1, e, node * 2 + 1);
        self.t[node] = self.t[node * 2] + self.t[node * 2 + 1];
    }

    fn query(&self, l: usize, r: usize, s: usize, e: usize, node: usize) -> i64 {
        if (r < s) || (e < l) {
            return 0;
        }
        if (l <= s) && (e <= r) {
            return self.t[node];
        }
        let mid = (s + e) / 2;
        return self.query(l, r, s, mid, node * 2) + self.query(l, r, mid + 1, e, node * 2 + 1);
    }
}
