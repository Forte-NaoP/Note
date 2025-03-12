# Firestore API 정리

[공식 문서](https://firebase.google.com/docs/firestore/manage-databases?hl=ko)

---

## Firestore

구글 Firebase 플랫폼의 NoSQL 문서 기반 데이터 베이스.

기존 SQL방식인 `Table, Row, Column` 대신 `Collection, Document`로 데이터를 구성함.

- `Collection`: 비슷한 성격을 가진 데이터의 집합
- `Document`: `Collection` 데이터의 기본 단위
  - 각 Document 는 고유 ID를 갖고 내부에 `Field`라는 이름의 Key-Value 데이터 \쌍을 저장함
- `Subcollection`: Document는  내부에 Collection을 가질 수 있음

---

## 인스턴스 초기화

```js
import { initializeApp } from "firebase/app";
import { getFirestore } from "firebase/firestore";

// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

const firebaseConfig = {
    /* FIREBASE_CONFIGURATION */
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);

// Initialize Cloud Firestore and get a reference to the service
const db = getFirestore(app);
```

혹은 CDN에서 추가

```html
<body>
  <!-- Insert this script at the bottom of the HTML, but before you use any Firebase services -->
  <script type="module">
    import { initializeApp } from 'https://www.gstatic.com/firebasejs/11.4.0/firebase-app.js'
    import { getFirestore } from 'https://www.gstatic.com/firebasejs/11.4.0/firebase-firestore.js'
  </script>
</body>
```

---

## FireStore

---

## 1. collection(db, "collectionName")

- **설명**: 특정 Firestore 컬렉션에 대한 참조(`CollectionReference`)를 생성  
- **사용 예시**: 
  ```js
  const colRef = collection(db, "articles");
  ```
- **주요 용도**: 다른 Firestore 연산(`addDoc`, `query` 등)을 위해 컬렉션을 가리킬 때 사용

## 2. getDocs(queryObject)

- **설명**: Firestore 쿼리(`Query`)나 컬렉션 참조(`CollectionReference`)를 기반으로 **여러 문서를 가져옴**  
- **반환값**: `QuerySnapshot` 객체 (해당 쿼리에 일치하는 문서들의 스냅샷 정보)  
- **사용 예시**:
  ```js
  const querySnapshot = await getDocs(query(colRef, orderBy("createdAt")));
  querySnapshot.forEach((doc) => {
    console.log(doc.id, doc.data());
  });
  ```

## 3. orderBy(fieldPath, directionStr?)

- **설명**: Firestore 쿼리에서 결과를 특정 필드를 기준으로 정렬할 때 사용  
- **매개변수**:
  - `fieldPath`: 정렬할 필드 이름 (예: `"createdAt"`)
  - `directionStr?`: 정렬 방향, `"asc"`(기본값) 또는 `"desc"`
- **사용 예시**:
  ```js
  const q = query(colRef, orderBy("createdAt", "desc"));
  const snapshot = await getDocs(q);
  ```

## 4. query(collectionRef, ...constraints)

- **설명**: Firestore 컬렉션 참조에 여러 **조건**(정렬, 필터, 제한 등)을 추가해 쿼리 객체를 생성  
- **반환값**: `Query` 객체  
- **사용 예시**:
  ```js
  import { query, where, orderBy } from "firebase/firestore";

  const q = query(
    collection(db, "articles"),
    where("author", "==", user.uid),
    orderBy("createdAt", "desc")
  );
  ```

## 5. getDoc(docRef)

- **설명**: Firestore 문서 참조(`DocumentReference`)를 통해 해당 문서 하나를 가져옴  
- **반환값**: `DocumentSnapshot` 객체  
- **사용 예시**:
  ```js
  const docRef = doc(db, "articles", "documentId");
  const docSnap = await getDoc(docRef);
  if (docSnap.exists()) {
    console.log(docSnap.data());
  }
  ```

## 6. doc(db, "collectionName", "docId")

- **설명**: 특정 Firestore 컬렉션 내부의 문서에 대한 참조(`DocumentReference`)를 생성  
- **사용 예시**:
  ```js
  const docRef = doc(db, "users", userId);
  ```
- **주요 용도**: 해당 문서에 `getDoc`, `updateDoc`, `deleteDoc` 등을 실행할 때 참조로 사용

## 7. Timestamp

- **설명**: Firestore에서 날짜/시간을 표현하기 위한 클래스  
- **주요 메서드**:
  - `Timestamp.now()`: 현재 시간을 Firestore 타임스탬프로 생성
  - `Timestamp.fromDate(date)`: 자바스크립트 Date 객체를 Firestore 타임스탬프로 변환
- **사용 예시**:
  ```js
  const now = Timestamp.now();
  const custom = Timestamp.fromDate(new Date("2025-01-01"));
  ```

## 8. addDoc(collectionRef, data)

- **설명**: 지정한 컬렉션 참조에 **새 문서**를 추가 (문서 ID는 **자동 생성**)  
- **반환값**: `DocumentReference` (생성된 문서의 참조)  
- **사용 예시**:
  ```js
  const newArticle = {
    title: "Hello Firestore",
    createdAt: Timestamp.now(),
  };
  const docRef = await addDoc(collection(db, "articles"), newArticle);
  console.log("Added new article with ID:", docRef.id);
  ```

## 9. updateDoc(docRef, data)

- **설명**: 특정 문서를 부분 업데이트 (문서가 존재하지 않으면 에러 발생)  
- **사용 예시**:
  ```js
  const docRef = doc(db, "articles", "articleId");
  await updateDoc(docRef, {
    title: "Updated Title",
    updatedAt: Timestamp.now()
  });
  ```
- 기존에 존재하던 필드 중 data에 지정되지 않은 필드들은 값이 유지된다.

## 10. deleteDoc(docRef)

- **설명**: 특정 문서를 Firestore에서 **삭제**  
- **사용 예시**:
  ```js
  const docRef = doc(db, "articles", "articleId");
  await deleteDoc(docRef);
  console.log("Document deleted");
  ```

---

## 참고 링크

- [Firebase Web Docs](https://firebase.google.com/docs/web)
- [Cloud Firestore 문서](https://firebase.google.com/docs/firestore)

---

## Firebase Auth

---

## 1. createUserWithEmailAndPassword(auth, email, password)

- **설명**: 이메일과 비밀번호를 사용해 새로운 계정을 생성 (회원가입)  
- **매개변수**:
  1. `auth`: 초기화된 Firebase Auth 객체
  2. `email`: 생성할 계정의 이메일
  3. `password`: 설정할 비밀번호
- **반환값**: `Promise<UserCredential>`  
  - 성공 시 `UserCredential.user`는 새로 생성된 **User** 객체를 담음  
- **사용 예시**:
  ```js
  import { createUserWithEmailAndPassword } from "firebase/auth";

  const userCredential = await createUserWithEmailAndPassword(auth, "test@example.com", "password123");
  console.log("Created user:", userCredential.user);
  ```

## 2. signInWithEmailAndPassword(auth, email, password)

- **설명**: 이메일과 비밀번호로 사용자 로그인(인증)  
- **매개변수**:
  1. `auth`: 초기화된 Firebase Auth 객체
  2. `email`: 로그인할 계정의 이메일
  3. `password`: 로그인할 계정의 비밀번호
- **반환값**: `Promise<UserCredential>`  
  - 성공 시 `UserCredential.user`는 로그인된 **User** 객체를 담음  
- **사용 예시**:
  ```js
  import { signInWithEmailAndPassword } from "firebase/auth";

  const userCredential = await signInWithEmailAndPassword(auth, "test@example.com", "password123");
  console.log("Signed in user:", userCredential.user);
  ```

## 3. signOut(auth)

- **설명**: 현재 로그인된 사용자를 로그아웃 시킴  
- **매개변수**: 
  1. `auth`: 초기화된 Firebase Auth 객체
- **반환값**: `Promise<void>` (로그아웃이 완료되면 완료)
- **사용 예시**:
  ```js
  import { signOut } from "firebase/auth";

  await signOut(auth);
  console.log("User signed out.");
  ```

## 4. onAuthStateChanged(auth, callback)

- **설명**: 인증 상태가 변경될 때마다(로그인·로그아웃·초기화 등) 콜백 함수를 호출
- **매개변수**:
  1. `auth`: 초기화된 Firebase Auth 객체
  2. `callback(user)`: 사용자 정보가 변할 때마다 실행되는 함수
     - `user`: 로그인되어 있으면 `User` 객체, 없으면 `null`
- **반환값**: **Unsubscribe** 함수  
  - 이 함수를 호출하면 `onAuthStateChanged` 구독을 해제할 수 있음
- **사용 예시**:
  ```js
  import { onAuthStateChanged } from "firebase/auth";

  onAuthStateChanged(auth, (user) => {
    if (user) {
      console.log("User is signed in:", user);
    } else {
      console.log("No user is signed in.");
    }
  });
  ```

---

## 추가 참고

- **UserCredential**: Firebase Auth가 로그인, 회원가입 등의 결과로 반환하는 인증 정보 객체
  - `user`: `User` 객체로, 현재 로그인된 사용자 정보 (uid, email, displayName 등)
  - `providerId`: 인증 제공자("password", "google.com" 등)
  - 기타 metadata

- **에러 처리**:
  - 모든 함수가 `Promise` 기반이므로, `try...catch`나 `.catch(...)`를 통해 **오류 코드**(예: `"auth/invalid-email"`, `"auth/email-already-in-use"`)를 처리할 수 있음.
