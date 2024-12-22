use std::vec;

struct SegTree {
    t: Vec<i64>,
    base: usize
}

impl SegTree {
    fn new(mut n: usize) -> Self {
        let mut b = 1;
        while n > 0 {
            b <<= 1;
            n >>= 1;
        }
        SegTree { t: vec![0; b << 1], base: b }
    }

    fn update_tb(&mut self, idx: usize, s: usize, e: usize, node: usize) {
        if (idx < s) || (e < idx) {
            return;
        }
        if s == e {
            self.t[node] = 1;
            return
        }
        let mid = (s + e) / 2;
        self.update_tb(idx, s, mid, node * 2);
        self.update_tb(idx, mid + 1, e, node * 2 + 1);
        self.t[node] = self.t[node * 2] + self.t[node * 2 + 1];
    }

    fn update_bt(&mut self, mut idx: usize, val: i64) {
        idx += self.base;
        let erase = self.t[idx];
        while idx > 0 {
            self.t[idx] ^= erase;
            self.t[idx] |= val;
            idx >>= 1;
        }
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

    fn query_bt(&self, mut l: usize, mut r: usize) -> i64 {
        let mut ret = 0;
        l += self.base;
        r += self.base;

        while l <= r {
            if l & 1 == 1 {
                ret |= self.t[l];
                l += 1;
            }
            if r & 1 == 0 {
                ret |= self.t[r];
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }

        ret
    }
}

struct SegTreeCmp<T>
where T: std::cmp::Ord + Copy {
    t: Vec<T>,
    base: usize,
    default: T,
    cmp: fn(T, T) -> T
}

impl<T> SegTreeCmp<T>
where T: std::cmp::Ord + Copy {
    fn new(mut n: usize, default: T, cmp: fn(T, T) -> T) -> Self {
        let mut b = 1;
        while n > 0 {
            b <<= 1;
            n >>= 1;
        }
        SegTreeCmp { t: vec![default; b << 1], base: b, default, cmp }
    }

    fn update_bu(&mut self, mut idx: usize, val: T) {
        idx += self.base;
        self.t[idx] = val;
        idx >>= 1;
        while idx > 0 {
            self.t[idx] = (self.cmp)(self.t[idx << 1], self.t[idx << 1 | 1]);
            idx >>= 1;
        }
    }

    fn query_td(&self, l: usize, r: usize) -> T {
        self._query_td(l, r, 0, self.base - 1, 1)
    }

    fn _query_td(&self, l: usize, r: usize, s: usize, e: usize, idx: usize) -> T {
        if r < s || e < l {
            return self.default;
        }
        if l <= s && e <= r {
            return self.t[idx];
        }
        let mid = (s + e) / 2;
        (self.cmp)(
            self._query_td(l, r, s, mid, idx << 1), 
            self._query_td(l, r, mid + 1, e, idx << 1 | 1)
        )
    }
}
