from typing import List

class MyHeap:
    def __init__(self, seq: List[int] | None = None):
        self.heap = [None]
        self.end = 1
        if seq:
            self.heap.extend(seq)
            self.end = len(self.heap)
            self.heappify()
            
    def __len__(self) -> int:
        return self.end - 1
    
    def __bool__(self):
        return self.end > 1

    def __repr__(self):
        return f'{self.heap[1:]}'

    def __down(self, cur):
        while cur < self.end:
            nxt = cur << 1
            right = nxt | 1
            if right < self.end and self.heap[nxt] > self.heap[right]:
                nxt = right
            if nxt < self.end and self.heap[cur] > self.heap[nxt]:
                self.heap[cur], self.heap[nxt] = self.heap[nxt], self.heap[cur]
                cur = nxt
            else:
                break

    def __up(self, cur):
        while cur > 1:
            parent = cur >> 1
            if self.heap[parent] <= self.heap[cur]:
                break
            self.heap[cur], self.heap[parent] = self.heap[parent], self.heap[cur]
            cur = parent

    def heappify(self):
        for cur in range(self.end - 1, 0, -1):
            self.__down(cur)

    def push(self, val: int):
        if len(self.heap) > self.end:
            self.heap[self.end] = val
        else:
            self.heap.append(val)
        self.end += 1
        self.__up(self.end - 1)
        
        
    def pop(self) -> int | None:
        if self.end <= 1:
            return None
        ret =  self.heap[1]
        self.end -= 1
        self.heap[1] = self.heap[self.end]
        self.__down(1)        
        return ret