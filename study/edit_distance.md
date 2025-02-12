# 편집 거리

문자열 A를 문자열 B로 바꾸는데 필요한 연산(문자 추가, 삭제, 교체)의 최소 횟수

`DP[i][j]: A[..i]를 B[..j]로 편집하는 연산의 최소 횟수`

`A[i] == B[j]`라면 해당 위치에 대해서는 편집할 필요가 없으므로 `DP[i][j] == DP[i - 1][j - 1]`

`A[i] != B[j]`라면 편집이 필요하다.

추가하는 경우는 `A[i]`에 문자열을 하나 추가해서 `B[j]`가 되어야 하므로 `DP[i][j - 1] + 1`, 즉 `A[..i]를 B[..j - 1]로 변경하는 최소 횟수`에 1을 더해준다.

변경은 `A[i]`를 `B[j]`로 바꾸는 것이므로 `DP[i - 1][j - 1] + 1`

삭제는 `DP[i - 1][j] + 1`이 된다.
(`A[..i - 1]`로 `B[j]`를 만들었으므로 `A[i]`를 삭제)
