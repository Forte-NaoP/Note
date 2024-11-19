from collections import defaultdict

g = {1: [2, 4], 2: [3], 3: [5], 4: [5], 5: {1}}

def tarzan(g, n):

    _label = 0
    label = [0 for _ in range(n + 1)]
    finished = [0 for _ in range(n + 1)]
    st = []
    scc_set = []
    scc_top = []

    def _tarzan(u):
        nonlocal _label

        _label += 1
        parent = label[u] = _label
        st.append(u)

        print(f'enter {u} p: {parent}')
        for v in g[u]:
            if label[v] == 0:
                parent = min(parent, _tarzan(v))
            elif finished[v] == 0:
                parent = min(parent, label[v])
        
        if parent == label[u]:
            scc = []
            while True:
                p = st.pop()
                finished[p] = u
                scc.append(p)
                if p == u:
                    scc_top.append(u)
                    break
            scc_set.append(scc)
        
        print(f'{u} return {parent}')
        return parent
    
    for i in range(1, n + 1):
        if label[i] == 0:
            _tarzan(i)

    indegree = defaultdict(int)
    for u in g.keys():
        for v in g[u]:
            if finished[u] != finished[v]:
                indegree[finished[v]] += 1
    print(scc_set)
    print(scc_top)
    print(label)
    print(finished)
    ans = 0
    for top in scc_top:
        if indegree[top] == 0:
            ans += 1
    return ans

tarzan(g, 5)