#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <set>

using namespace std;

int parent[101];
int u_rank[101];

int find(int x) {
    if (x == parent[x]) return x;
    return parent[x] = find(parent[x]);
}

void joint(int x, int y) {
    x = find(x);
    y = find(y);

    if (x == y) return;
    if (u_rank[x] >= u_rank[y]) swap(x, y);
    parent[x] = y;
    if (u_rank[x] == u_rank[y]) u_rank[y] = u_rank[x] + 1; 
}