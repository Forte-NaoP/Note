# HTML 정리

---

## 태그

- `h{1~6}`: 제목
- 리스트
  - `ul`: 순서 없는 리스트
  - `ol`: 순서 있는 리스트
  - `li`: 리스트의 항목
- `a`: 앵커, href 속성으로 다른 페이지, 페이지 내 다른 곳, 이메일 보내기 가능
- `p`: 문단 태그
- `input`: 사용자 입력
  - 속성:
    - `type`: 입력 타입 (`text`, `password`, `radio`, `checkbox`, `submit`, `button`)
      - `checkbox`, `radio`: `name` 속성을 통해 그룹화 가능
      - `submit`은 제출 버튼, `onclick` 기본 값은 페이지 새로고침
    - `value`: 기본값
    - `placeholder`: 안내문
- `form`: 서버로 값을 전송하기 위한 태그
- `select`: 선택 요소
  - `option` 태그로 값을 선택
  - `option` 태그의 `selected` 속성으로 기본값 지정 가능

  ```html
  <select>
    <option value="1" selected>1</option>
    <option value="2">2</option>
  </select>
  ```

- `iframe`: 현재 문서에 다른 html 문서 삽입
  - `src`: 삽입할 문서 주소
  - `width`, `height`: 너비, 높이

- `div`: 레이아웃 구분용, 개행 발생
- `span`: 레이아웃 구분용, 개행 없음

---

## 시멘틱 태그

웹 페이지의 구조와 내용을 의미적으로 표현할 수 있도록 고안된 태그

- `header`: 페이지나 섹션의 머리말, 로고, 제목, 탐색 메뉴등을 포함
- `nav`: 사이트 내의 주요 내비게이션 링크들을 감싸는 태그
- `main`: 문서의 주요 콘텐츠 영역, 페이지 내에서 유일
- `section`: 문서 내에서 주제별로 구분되는 영억
- `article`: 독립적으로 구분되는 콘텐츠
- `aside`: 본문과 간접적으로 관련된 보조 콘텐츠. ex) 사이드바, 광고

---

## CSS

### 사용법

- `head` 내에서 `style` 태그로 정의
- `link` 태그로 외부 참조

  ```html
  <link href="./styles.css" rel="stylesheet">
  ```

- inline: 각각 태그마다 적용

  ```html
  <div style="color: red;"></div>
  ```

### 선택자

```css
* {} /*전체 선택자, 모든 태그 적용*/
div {} /*태그 선택자, 특정 태그만 적용*/
.class1.class2 {} /*클래스 선택자 AND 조건*/
.class1, .class2 {} /*클래스 선택자 OR 조건*/
#id {} /*id 선택자*/
```

---

### CSS 속성

- font
  - `color`, `font-size`, `font-weight`
- `text-align`: 텍스트 정렬
- `border`: 두께 종류 색상
- `background-color`: 요소의 배경색
- `border-radius`
- `margin`: `border` 바깥쪽 여백
- `padding`: `border` 안쪽 여백
- `background-image: url("");`: 배경 이미지
- `background-size`: `auto`, `cover`, `contain`

---

### `display: flex`

#### 1. 기본 개념

- **Flex Container(플렉스 컨테이너):**  
  `display: flex` 또는 `display: inline-flex`를 지정한 요소가 플렉스 컨테이너가 됩니다. 이 컨테이너 내부에 있는 **직접 자식 요소들**은 자동으로 **플렉스 아이템**이 됩니다.

- **Flex Items(플렉스 아이템):**  
  플렉스 컨테이너의 직접 자식 요소들이며, 컨테이너 내에서 자유롭게 정렬, 순서 변경, 크기 조절 등이 가능합니다.

---

#### 2. Flex Container 관련 주요 속성

Flex 컨테이너에 적용할 수 있는 대표적인 속성들은 다음과 같습니다:

##### 2.1. `flex-direction`

- **역할:** 플렉스 아이템들이 주 축(main axis)을 따라 어떻게 배치될지를 결정합니다.
- **값:**
  - `row` (기본값): 왼쪽에서 오른쪽으로 수평 배치
  - `row-reverse`: 오른쪽에서 왼쪽으로 수평 배치
  - `column`: 위에서 아래로 수직 배치
  - `column-reverse`: 아래에서 위로 수직 배치

##### 2.2. `flex-wrap`

- **역할:** 플렉스 아이템들이 한 줄에 다 들어가지 않을 경우, 다음 줄로 넘어가게 할지 여부를 결정합니다.
- **값:**
  - `nowrap` (기본값): 한 줄에 모두 배치 (넘치면 축소됨)
  - `wrap`: 필요시 여러 줄로 배치
  - `wrap-reverse`: 반대 방향(역순)으로 줄바꿈

##### 2.3. `justify-content`

- **역할:** **주 축(main axis)**을 기준으로 아이템들을 정렬하고, 남은 공간을 분배합니다.
- **값:**
  - `flex-start` (기본값): 시작 부분에 정렬
  - `flex-end`: 끝 부분에 정렬
  - `center`: 중앙 정렬
  - `space-between`: 아이템들 사이에 동일한 간격 배분 (양 끝은 붙음)
  - `space-around`: 아이템 주위에 동일한 간격 배분 (양 끝에 절반 간격)
  - `space-evenly`: 모든 간격을 균등하게 배분

##### 2.4. `align-items`

- **역할:** **교차 축(cross axis)**을 기준으로 아이템들을 정렬합니다.
- **값:**
  - `stretch` (기본값): 아이템들이 컨테이너의 높이나 너비에 맞게 늘어남
  - `flex-start`: 시작 부분에 정렬
  - `flex-end`: 끝 부분에 정렬
  - `center`: 중앙 정렬
  - `baseline`: 텍스트의 기준선에 맞춰 정렬

##### 2.5. `align-content`

- **역할:** 아이템들이 여러 줄로 배치된 경우, **교차 축**을 기준으로 전체 줄들을 정렬합니다.
- **값:** `flex-start`, `flex-end`, `center`, `space-between`, `space-around`, `stretch`  
  *(참고: 단일 줄인 경우에는 효과가 없습니다.)*

---

#### 3. Flex Item 관련 주요 속성

플렉스 아이템에 적용할 수 있는 주요 속성들은 다음과 같습니다:

##### 3.1. `order`

- **역할:** 아이템들의 배치 순서를 지정합니다.
- **특징:** 기본값은 `0`이며, 값이 작을수록 먼저 배치됩니다. 숫자가 같으면 HTML 코드 내의 순서를 따릅니다.

##### 3.2. `flex-grow`

- **역할:** 남은 공간을 아이템들이 얼마나 확장하여 채울지를 결정합니다.
- **특징:** 기본값은 `0`으로, 값이 클수록 더 많이 확장됩니다.
  
  ```css
  .item {
    flex-grow: 1; /* 모든 아이템이 동일한 비율로 확장 */
  }
  ```

##### 3.3. `flex-shrink`

- **역할:** 컨테이너의 공간이 부족할 때 아이템들이 얼마나 축소될지를 결정합니다.
- **특징:** 기본값은 `1`. 값이 클수록 더 많이 축소됩니다.

##### 3.4. `flex-basis`

- **역할:** 아이템의 기본 크기를 지정합니다. (주 축 방향의 초기 크기)
- **값:** `auto` (기본값) 또는 특정 길이(예: `200px`, `20%` 등)

##### 3.5. `flex` (축약 속성)

- **역할:** `flex-grow`, `flex-shrink`, `flex-basis`를 한 번에 지정하는 축약 속성입니다.
- **예시:**
  
  ```css
  .item {
    flex: 1 1 200px;
  }
  ```
  
  위 코드는 `flex-grow: 1; flex-shrink: 1; flex-basis: 200px;`와 동일합니다.

##### 3.6. `align-self`

- **역할:** 개별 아이템에 대해 컨테이너의 `align-items` 속성을 재정의하여, 해당 아이템만 따로 정렬할 수 있습니다.
- **값:** `auto` (기본값, 컨테이너의 `align-items`를 따름), `flex-start`, `flex-end`, `center`, `baseline`, `stretch`

---
