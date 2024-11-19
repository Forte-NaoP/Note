# 위상정렬

비순환 유향 그래프(DAG)에 대해서 정점을 선형으로 정렬하는 것

각 노드에 선행 조건이 존재할 때 사용 가능

## 정렬 방법

1. 각 노드의 진입 차수 기록
2. 진입 차수가 0인 정점을 큐에 삽입
3. 큐에 담긴 정점에서 나가는 간선을 제거하고 해당 간선에 연결된 정점의 진입 차수를 감소
4. 2-3과정을 큐가 비거나 모든 정점을 탐색할 때까지 반복

## 구현

```python
graph = defaultdict(list)
indegree = [0] * (n + 1)

# 인접 리스트 및 진입 차수 초기화
# for _ in range(k):
#     a, b = map(int, input().split())
#     graph[a].append(b)
#     indegree[b] += 1

# 진입 차수 0인 정점 큐에 추가
q = deque(filter(lambda x: indegree[x] == 0, range(1, n + 1)))

while q:
    cur = st.popleft()
    print(q)

    if cur in graph:
        for nxt in graph[cur]:
            indegree[nxt] -= 1
            if indegree[nxt] == 0:
                st.append(nxt)
```
