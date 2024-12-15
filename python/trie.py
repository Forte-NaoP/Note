from typing import List

class Node:
    def __init__(self):
        self.next: List[Node | None] = [None] * 26
        self.end = False
        self.count = 0

class Trie:
    def __init__(self):
        self.root = Node()

    def insert(self, s: str):
        node = self.root
        for ch in s:
            ch = ord(ch) - ord('a')
            if not node.next[ch]:
                node.next[ch] = Node()
            node = node.next[ch]
            node.count += 1
        node.end = True
    
    def search(self, s:str):
        node = self.root
        for i, ch in enumerate(s):
            ch = ord(ch) - ord('a')
            if node.next[ch].count == 1:
                return i + 1
            node = node.next[ch]
        return len(s)

    def print(self):
        result = []
        self.dfs(self.root, "", result)
        print(result)

    def dfs(self, node, word, result):
        if node.end:
            result.append(word)
        for i in range(26):
            if node.next[i]:
                self.dfs(node.next[i], word+str(chr(i+ord('a'))), result)
