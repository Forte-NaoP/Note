# 정렬

## Bubble Sort

인접한 두 원소를 비교하여 역순이면 교환한다.

```python
# seq = []
# 총 n번 반복
for i in range(n - 1, -1, -1):
    is_swapped = False
    # 매 반복 후에 seq[i + 1:]은 정렬된 상태이므로 
    # seq[:i] 까지만 확인
    for j in range(i):
        if seq[j] > seq[j + 1]:
            seq[j], seq[j + 1] = seq[j + 1] > seq[j]
            is_swapped = True
    if not is_swapped:
        break
```

## Selection Sort

현재 `i = [0, n)`일 때 `seq[i:]`에서 가장 작은 원소와 seq[i]를 교환한다.

```python
# seq = []
for cur in range(n):
    min_index = cur
    for i in range(cur, n):
        if seq[min_index] > seq[i]:
            min_index = i
    seq[min_index], seq[cur] = seq[cur], seq[min_index]
```

## Insertion Sort

현재 `i = [1, n)`일 때 seq[:i]에서 seq[i]를 삽입할 위치를 찾아 삽입한다.

```python
# seq = []
for cur in range(1, n):
    while cur > 0 and seq[cur] < seq[cur - 1]:
        seq[cur], seq[cur - 1] = seq[cur - 1], seq[cur]
        cur -= 1
```

## Quick Sort

pivot을 기준으로 배열을 양쪽으로 나누어 정렬한다.

```python
# seq = []
def partition(left, right):
    # [left, right] 범위를 재배치
    pivot = seq[(left + right) // 2]

    while left <= right:
        # left를 pivot보다 큰 원소가 나타나는 지점까지 이동
        while seq[left] < pivot:
            left += 1
        # right를 pivot보다 작은 원소가 나타나느 지점까지 이동
        while seq[right] > pivot:
            right -= 1
        
        # 재배치가 안끝났으면
        if left <= right:
            # seq[left]는 pivot보다 크고 
            # seq[right]는 pivot보다 작으므로 
            # 서로 교환
            seq[left], seq[right] = seq[right], seq[left]
            # 한 칸씩 이동
            left, right = left + 1, right - 1

    return left

def quick_sort(left, right):
    print(seq, left, right)
    if left >= right:
        return
    
    mid = partition(left, right)
    quick_sort(left, mid - 1)
    quick_sort(mid, right)
```

## Merge Sort

배열을 두 부분으로 나누고 각 배열을 정렬한 뒤 하나로 합친다.

```python
# seq = []
def merge(left, mid, right):
    # seq[left:mid + 1]과 seq[mid + 1:right]를 합침
    tmp = []
    i, j = left, mid + 1
    # 두 부분 배열에서 작은 값을 채워 넣음
    while i <= mid and j <= right:
        if seq[i] < seq[j]:
            tmp.append(seq[i])
            i += 1
        else:
            tmp.append(seq[j])
            j += 1
    # 부분 배열에서 남은 것을 뒤에 추가함
    while i <= mid: 
        tmp.append(seq[i])
        i += 1
    while j <= right:
        tmp.append(seq[j])
        j += 1
    # 정렬된 배열을 원본 배열에 옮김
    seq[left:left+len(tmp)] = tmp[:]

def merge_sort(left, right):
    # 길이 2 이상의 배열에 대해 분할
    if left >= right:
        return
    mid = (left + right) // 2
    # 두 부분으로 나눔
    merge_sort(left, mid)
    merge_sort(mid + 1, right)
    # 나눈 배열을 합침
    merge(left, mid, right)
```
