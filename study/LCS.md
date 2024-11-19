# 최장 공통 부분 수열(LCS: Longest Common Subsequence)

`DP[i][j]: A[..i], B[..j]까지의 LCS길이`

- `A[i] == B[j]`: `DP[i][j] = DP[i - 1][j - 1] + 1`

- `A[i] != B[j]`: `DP[i][j] = max(DP[i - 1][j], DP[i][j - 1])`
