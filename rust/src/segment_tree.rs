pub trait Item: Default + Clone {
    fn operation(a: &Self, b: &Self) -> Self;
}

pub struct SegmentTree<T: Item> {
    base: usize,
    node: Vec<T>,
}

impl<T: Item> SegmentTree<T> {
    pub fn new(size: usize) -> Self {
        let mut base = 1;
        while base < size {
            base <<= 1;
        }

        let node = vec![T::default(); base << 1];
        SegmentTree { base, node }
    }

    pub fn update(&mut self, mut idx: usize, val: T) {
        idx += self.base;
        self.node[idx] = T::operation(&self.node[idx], &val);
        idx >>= 1;
        while idx > 0 {
            let left = &self.node[idx << 1];
            let right = &self.node[idx << 1 | 1];
            self.node[idx] = T::operation(left, right);
            idx >>= 1;
        }
    }

    pub fn query(&self, l: usize, r: usize) -> T {
        self._query(l, r, 0, self.base - 1, 1)
    }

    fn _query(&self, l: usize, r: usize, s: usize, e: usize, idx: usize) -> T {
        if r < s || e < l {
            return T::default();
        }

        if l <= s && e <= r {
            return self.node[idx].clone();
        }

        let mid = (s + e) / 2;
        let left = self._query(l, r, s, mid, idx << 1);
        let right = self._query(l, r, mid + 1, e, idx << 1 | 1);
        T::operation(&left, &right)
    }
}

pub trait RangeOp: Default + Clone {
    type D: Default + Clone + PartialEq;
    type U: Default + Clone + PartialEq;

    fn combine(a: &Self::D, b: &Self::D) -> Self::D;
    fn apply(node: &Self::D, upd: &Self::U, len: usize) -> Self::D;
    fn compose(old: &Self::U, new: &Self::U) -> Self::U;
}

pub struct SegmentTreeLazy<Op>
where
    Op:RangeOp,
{
    base: usize,
    node: Vec<Op::D>,
    lazy: Vec<Op::U>,
}

impl<Op> SegmentTreeLazy<Op>
where
    Op: RangeOp,
{
    pub fn new(size: usize) -> Self {
        let mut base = 1;
        while base < size {
            base <<= 1;
        }
        let capacity = base << 1;
        SegmentTreeLazy { 
            base,
            node: vec![Op::D::default(); capacity], 
            lazy: vec![Op::U::default(); capacity],
        }
    }

    fn propagate(&mut self, s: usize, e: usize, idx: usize) {
        let upd = std::mem::take(&mut self.lazy[idx]);
        if upd != Op::U::default() {
            self.node[idx] = Op::apply(&self.node[idx], &upd, e - s + 1);

            if s != e {
                let left = idx << 1;
                let right = idx << 1 | 1;

                self.lazy[left] = Op::compose(&self.lazy[left], &upd);
                self.lazy[right] = Op::compose(&self.lazy[right], &upd);
            }
        }
    }

    pub fn update_range(&mut self, l: usize, r: usize, upd: Op::U) {
        self._update(l, r, upd, 0, self.base - 1, 1);
    }

    fn _update(&mut self, l: usize, r: usize, upd: Op::U, s: usize, e: usize, idx: usize) {
        self.propagate(s, e, idx);
        if r < s || e < l {
            return;
        }
        if l <= s && e <= r {
            self.lazy[idx] = Op::compose(&self.lazy[idx], &upd);
            self.propagate(s, e, idx);
            return;
        }
        let mid = (s + e) >> 1;
        self._update(l, r, upd.clone(), s, mid, idx << 1);
        self._update(l, r, upd, mid + 1, e, idx << 1 | 1);
        self.node[idx] = Op::combine(
            &self.node[idx << 1],
            &self.node[idx << 1 | 1],
        );
    }

    pub fn query_range(&mut self, l: usize, r: usize) -> Op::D {
        self._query(l, r, 0, self.base - 1, 1)
    }

    fn _query(&mut self, l: usize, r: usize, s: usize, e: usize, idx: usize) -> Op::D {
        self.propagate(s, e, idx);
        if r < s || e < l {
            return Op::D::default();
        }
        if l <= s && e <= r {
            return self.node[idx].clone();
        }
        let mid = (s + e) >> 1;
        let left  = self._query(l, r, s, mid, idx << 1);
        let right = self._query(l, r, mid + 1, e, idx << 1 | 1);
        Op::combine(&left, &right)
    }
}

#[derive(Default, Clone, PartialEq)]
struct SumAdd;

impl RangeOp for SumAdd {
    type D = i64;
    type U = i64;

    fn combine(a: &Self::D, b: &Self::D) -> Self::D {
        *a + *b
    }

    fn apply(node: &Self::D, upd: &Self::U, len: usize) -> Self::D {
        *node + (*upd) * (len as i64)
    }

    fn compose(old: &Self::U, new: &Self::U) -> Self::U {
        *old + *new
    }
}
