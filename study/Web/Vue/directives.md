# Vue directives

---

## v-model  
**기능:**  
양방향 데이터 바인딩을 통해 사용자 입력과 상태를 자동으로 동기화합니다.

**Composition API 예시:**  
```html
<template>
  <input v-model="message" placeholder="메시지를 입력하세요" />
  <p>입력한 메시지: {{ message }}</p>
</template>

<script setup>
import { ref } from 'vue'
const message = ref('')
</script>
```

**설명:**  
- `<script setup>`에서 `ref`를 사용해 반응형 변수 `message`를 선언합니다.  
- 템플릿에서는 `v-model`을 통해 `<input>` 요소와 `message`가 연결되어, 사용자의 입력에 따라 값이 업데이트되고, 값이 변경되면 화면에도 즉시 반영됩니다.

---

## v-bind  
**기능:**  
HTML 속성이나 컴포넌트 props에 동적으로 데이터를 바인딩합니다.
`v-bind:`를 축약하여 `:`로 쓸 수 있습니다.

**Composition API 예시:**  
```html
<template>
  <img :src="imageSrc" alt="동적 이미지" />
  <button :disabled="isDisabled">버튼</button>
</template>

<script setup>
import { ref } from 'vue'
const imageSrc = ref('https://example.com/image.png')
const isDisabled = ref(false)
</script>
```

**설명:**  
- `:src`와 `:disabled`를 사용하여 각각 `imageSrc`와 `isDisabled` 변수의 값에 따라 동적으로 속성이 결정됩니다.
- 값이 변경되면 DOM에 자동 반영되어 실시간으로 업데이트됩니다.

---

## v-on  
**기능:**  
DOM 이벤트(예: 클릭)를 청취하여 특정 메소드를 실행합니다.
`v-on:`을 축약하여 `@`로 쓸 수 있습니다.

**Composition API 예시:**  
```html
<template>
  <button @click="handleClick">클릭하세요</button>
</template>

<script setup>
function handleClick() {
  console.log('버튼이 클릭되었습니다!')
}
</script>
```

**설명:**  
- `@click` 디렉티브를 사용해 버튼 클릭 시 `handleClick` 함수가 호출됩니다.
- Composition API에서는 `<script setup>` 내부에서 바로 함수를 정의하여 템플릿에서 사용할 수 있습니다.

---

## v-for  
**기능:**  
배열이나 객체를 순회하면서 리스트 형태의 DOM 요소를 생성합니다.  
**중요:** 각 반복 요소에는 고유의 `key`를 지정하여 효율적인 렌더링을 돕습니다.

**Composition API 예시:**  
```html
<template>
  <ul>
    <li v-for="item in items" :key="item.id">
      {{ item.name }}
    </li>
  </ul>
</template>

<script setup>
import { ref } from 'vue'
const items = ref([
  { id: 1, name: '항목 1' },
  { id: 2, name: '항목 2' },
  { id: 3, name: '항목 3' }
])
</script>
```

**설명:**  
- `items` 배열은 `ref`를 이용해 반응형으로 선언됩니다.
- 템플릿에서는 `v-for`를 사용해 배열의 각 객체를 순회하며, 각 `<li>` 요소에 `:key`로 고유 식별자를 부여합니다.

---

## v-if  
**기능:**  
주어진 조건에 따라 요소를 렌더링하거나 제거합니다.

**Composition API 예시:**  
```html
<template>
  <div v-if="isVisible">
    이 콘텐츠는 isVisible이 true일 때만 보입니다.
  </div>
  <div v-else>
    콘텐츠가 보이지 않습니다.
  </div>
</template>

<script setup>
import { ref } from 'vue'
const isVisible = ref(true)
</script>
```

**설명:**  
- `isVisible` 변수를 이용하여 조건을 제어합니다.
- 조건이 참이면 첫 번째 `<div>`가 렌더링되고, 그렇지 않으면 `v-else` 블록의 내용이 보여집니다.
- Composition API 방식에서는 반응형 변수로 선언된 값을 직접 템플릿 내 조건문에 사용할 수 있습니다.

---