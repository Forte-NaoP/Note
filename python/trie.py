from typing import List

def encode(s):
    val = 0
    for c in s:
        val <<= 5
        val += ord(c) - ord('A') + 1
    return val

def decode(val):
    s = []
    while val > 0:
        s.append(chr((val & 31) + ord('A') - 1))
        val >>= 5
    s.reverse()
    return ''.join(s)

class Node:
    def __init__(self):
        self.link: List[Node, None] = [None for _ in range(26)]
        self.end = False

    def insert(self, s: int):
        if not s:
            self.end = True
            return
        idx = (s & 31) - 1
        if not self.link[idx]:
            self.link[idx] = Node()
        self.link[idx].insert(s >> 5)
    
    def find(self, s: int):
        if not s:
            return True if self.end else False
        idx = (s & 31) - 1
        if not self.link[idx]:
            return False
        return self.link[idx].find(s >> 5)
