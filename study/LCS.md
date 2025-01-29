# 최장 공통 부분 수열(LCS: Longest Common Subsequence)

---

## LCS 길이 구하기

`DP[i][j]: A[..i], B[..j]까지의 LCS길이`

- `A[i] == B[j]`: `DP[i][j] = DP[i - 1][j - 1] + 1`

- `A[i] != B[j]`: `DP[i][j] = max(DP[i - 1][j], DP[i][j - 1])`

---

## LCS 문자열 구하기

1. **역추적 (Backtracking) 방식**  
   `DP` 테이블을 이용해 `(N, M)`에서 `(0, 0)`까지 역추적하여 LCS를 복원.
   - `A[i] == B[j]`이면 LCS에 포함됨 → `i-1, j-1`로 이동  
   - `DP[i-1][j]`, `DP[i][j-1]`를 비교해 큰 쪽으로 이동  

```python
def LCS(A: str, B: str) -> str:
    N, M = len(A), len(B)
    DP = [[0] * (M + 1) for _ in range(N + 1)]

    # DP 테이블 채우기
    for i in range(1, N + 1):
        for j in range(1, M + 1):
            if A[i - 1] == B[j - 1]:  # 문자가 같으면
                DP[i][j] = DP[i - 1][j - 1] + 1
            else:
                DP[i][j] = max(DP[i - 1][j], DP[i][j - 1])

    # LCS 역추적 (Backtracking)
    i, j = N, M
    lcs = []
    while i > 0 and j > 0:
        if A[i - 1] == B[j - 1]:  # 같은 문자는 LCS에 포함
            lcs.append(A[i - 1])
            i -= 1
            j -= 1
        elif DP[i - 1][j] > DP[i][j - 1]:  # 위쪽이 크다면 위로 이동
            i -= 1
        else:  # 왼쪽이 크다면 왼쪽으로 이동
            j -= 1

    return ''.join(reversed(lcs))  # 역순으로 정렬하여 반환
```

---

### **시간 복잡도**

- `DP` 테이블을 채우는 과정: **O(N × M)**
- `LCS` 역추적 과정: **O(N + M)**
- 전체 시간 복잡도: **O(N × M)**
