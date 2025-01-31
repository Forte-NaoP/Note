from typing import List

class Node:
    def __init__(self):
        self.next: List[Node | None] = [None] * 26
        self.end = False

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
        node.end = True
    
    def search(self, s:str):
        node = self.root
        for ch in s:
            ch = ord(ch) - ord('a')
            if not node.next[ch]:
                return False
            node = node.next[ch]
        return node.end

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
