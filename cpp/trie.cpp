#include <cstdio>
#include <cstdlib>
#include <algorithm>
#include <iostream>
#include <cstring>
#include <vector>
#include <string>

using namespace std;

struct TrieNode {
    TrieNode* children[26]; // 26개의 자식 노드
    bool isEnd; // 단어의 끝을 나타내는 플래그

    TrieNode() {
        isEnd = false;
        for (int i = 0; i < 26; i++) children[i] = nullptr;
    }
};

class Trie {
private:
    TrieNode* root;

public:
    Trie() { root = new TrieNode(); }

    // 단어 삽입
    void insert(const string& word) {
        TrieNode* node = root;
        for (char c : word) {
            int index = c - 'A';
            if (!node->children[index]) {
                node->children[index] = new TrieNode();
            }
            node = node->children[index];
        }
        node->isEnd = true;
    }

    // 단어 검색
    bool search(const string& word) {
        TrieNode* node = root;
        for (char c : word) {
            int index = c - 'A';
            if (!node->children[index]) return false;
            node = node->children[index];
        }
        return node->isEnd;
    }

    // 접두사 검색
    bool startsWith(const string& prefix) {
        TrieNode* node = root;
        for (char c : prefix) {
            int index = c - 'A';
            if (!node->children[index]) return false;
            node = node->children[index];
        }
        return true;
    }

    ~Trie() {
        deleteNode(root);
    }

private:
    // 동적 메모리 해제 (트라이 트리 전체 삭제)
    void deleteNode(TrieNode* node) {
        if (!node) return;
        for (int i = 0; i < 26; i++) {
            deleteNode(node->children[i]);
        }
        delete node;
    }
};
