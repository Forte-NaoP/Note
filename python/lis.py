import sys
from collections import deque

input = lambda : sys.stdin.readline().strip()

from bisect import bisect_left

INF = 10 ** 9

n = int(input())
seq = list(map(int, input().split()))
parent = [[-1, -1] for _ in range(n)]
find_min = [INF for _ in range(n)]
end = 0

for i, num in enumerate(seq):
    idx = bisect_left(find_min, num, 0, end)
    if idx == end:
        find_min[end] = num
        end += 1
    else:
        find_min[idx] = num
    parent[i][:] = (idx, num)

lis = deque()
last = end - 1
idx = n - 1
while idx > -1 and last > -1:
    if parent[idx][0] == last:
        lis.appendleft(parent[idx][1])
        last -= 1
    idx -= 1

print(*lis)