#include <cstdio>
#include <cstdlib>
#include <algorithm>
#include <iostream>
#include <array>
#include <vector>
#include <cmath>

using namespace std;

#define MAX_H 16

vector<int> graph[10001];
int parent[10001][MAX_H];
int depth[10001];
int sub_cnt[10001];

int init_tree(int node, int ancestor) {
    depth[node] = depth[ancestor] + 1;

    for (int& ch : graph[node]) {
        if (ch != ancestor) {
            parent[ch][0] = node;
            init_tree(ch, node);
        }
    }

    return 0;
}

void search_parent(int n) {
    for (int j = 1; j < MAX_H; ++j) {
        for (int i = 1; i <= n; ++i) {
            parent[i][j] = parent[parent[i][j-1]][j-1];
        }
    }
}

int find_lca(int a, int b) {
    if (depth[a] < depth[b]) swap(a, b);

    int diff = depth[a] - depth[b];

    for (int i=0; i<MAX_H; ++i) {
        if (diff & (1 << i)) a = parent[a][i];
    }

    if (a != b) {
        for (int i=MAX_H-1; i>=0; --i) {
            if (parent[a][i] != parent[b][i]) {
                a = parent[a][i];
                b = parent[b][i];
            }
        }
        a = parent[a][0];
    }
    return a;
}

int main() {

    freopen("input.txt", "rt", stdin);

    int T;
    scanf("%d", &T);
    int v, e, a, b;
    int p, d;
    int bn = 0;

    for (int i=1; i<=T; ++i) {
        for (int j=0; j<=bn; ++j) graph[j].clear();
        scanf("%d%d%d%d", &v, &e, &a, &b);
        for (int j=0; j<e; ++j) {
            scanf("%d%d", &p, &d);
            graph[p].push_back(d);
            graph[d].push_back(p);
        }
        
        init_tree(1, 0);
        search_parent(v);
        int lca = find_lca(a, b);
        printf("#%d %d %d\n", i, lca, 0);
        bn = v;
    }
}

