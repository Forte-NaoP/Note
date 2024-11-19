max_depth = 1
tree = dict()
depth = []
parent = []

def init(cur, p):
    depth[cur] = depth[p] + 1
    parent[cur][0] = p

    for i in range(1, max_depth + 1):
        parent[cur][i] = parent[parent[cur][i - 1]][i - 1]

    for nxt in tree[cur]:
        if nxt == p:
            continue
        init(nxt, cur)

def query(a, b):
    if a == 1 or b == 1:
        return 1
    
    if depth[a] > depth[b]:
        a, b = b, a

    if depth[a] != depth[b]:
        for i in range(max_depth, -1, -1):
            if depth[parent[b][i]] >= depth[a]:
                b = parent[b][i]
    
    lca = a
    if a != b:
        for i in range(max_depth, -1, -1):
            if parent[a][i] != parent[b][i]:
                a = parent[a][i]
                b = parent[b][i]
            lca = parent[a][i]

    return lca