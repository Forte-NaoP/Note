class SegTree():
    def __init__(self, n, arr, func, default):
        size = 1
        while size < n:
            size <<= 1
        self.size = size << 1
        self.base = size
        self.func = func
        self.default = default
        self.tree = [default for _ in range(self.size)]
        for i in range(n):
            self.update(i, arr[i])

    def update(self, idx, val):
        idx += self.base
        self.tree[idx] = val
        idx >>= 1
        while idx > 1:
            self.tree[idx] = self.func(self.tree[idx * 2], self.tree[idx * 2 + 1])
            idx >>= 1
    
    def query(self, l, r):
        return self.__query(0, self.base, 1, l, r)
    
    def __query(self, s, e, idx, l, r):
        if e < l or s > r:
            return self.default
        if l <= s and e <= r:
            return self.tree[idx]
        mid = (s + e) // 2
        return self.func(
            self.__query(s, mid, idx * 2, l, r),
            self.__query(mid + 1, e, idx * 2 + 1, l, r)
        )

    def __query(self, l, r):
        val = self.default
        l += self.base
        r += self.base

        while l < r:
            if l & 1 != 0:
                val = self.func(val, self.tree[l])
                l += 1
            l >>= 1

            if r & 1 == 0:
                val = self.func(val, self.tree[r])
                r -= 1
            r >>= 1
        
        return val