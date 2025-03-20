# Database MEMO

## SQL 구문 해석 순서

1. `FROM`: 데이터 소스를 지정
2. `ON`: 조인 조건을 설정
3. `JOIN`: 필요한 테이블 결합
4. `WHERE`: 행을 필터링
5. `GROUP BY`: 데이터를 그룹화
6. `HAVING`: 그룹화된 데이터에 추가 조건 적용
7. `SELECT`: 선택한 컬럼을 가져옴
8. `DISTINCT`: 중복된 행 제거
9. `ORDER BY`: 결과를 정렬
10. `LIMIT`: 결과의 개수를 제한

## 구문 별 문법

- `LIMIT` B, A | `LIMIT` A `OFFSET` B: B번째 행부터 A개의 행 선택 (0-index)

## ACID

