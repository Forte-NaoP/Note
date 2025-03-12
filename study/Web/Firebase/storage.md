# Firebase Storage API 정리

[공식 문서](https://firebase.google.com/docs/firestore/manage-databases?hl=ko)

---

## Storage

Firebase에서 제공하는 클라우드 기반 파일 저장 서비스.

Google Cloud Storage를 기반으로 하며, Firebase Authentication과 연동되어 사용자의 인증 상태에 따른 파일 접근 제어가 가능하다.

---

## 1. getStorage

```js
getStorage(app?: FirebaseApp, bucketUrl?: string): FirebaseStorage
```

- **역할:**  
  Firebase Storage 서비스를 초기화하고 인스턴스를 반환한다.  
  - 기본 Firebase 앱에 연결된 Storage 인스턴스를 얻거나, 특정 앱 인스턴스에 대해 사용 가능하다.
- **사용 예:**  

  ```js
  import { getStorage } from "firebase/storage";
  const storage = getStorage(); // 기본 앱의 Storage 인스턴스 사용
  ```

---

### 2. ref

```js
ref(storage: FirebaseStorage, url?: string): StorageReference
```

- **역할:**  
  Storage 내 특정 위치(파일 또는 폴더)를 가리키는 참조(reference)를 생성한다.

- **사용 예:**  

  ```js
  import { ref } from "firebase/storage";
  const fileRef = ref(storage, 'uploads/myImage.png');
  ```

---

### 3. uploadBytes

```js
uploadBytes(
    ref: StorageReference, 
    data: Blob | Uint8Array | ArrayBuffer, 
    metadata?: UploadMetadata
): Promise<UploadResult>
```

- **역할:**  
  지정한 참조(ref)를 통해 파일이나 Blob 데이터를 Firebase Storage에 업로드한다.
- **사용 예:**  

  ```js
  import { uploadBytes } from "firebase/storage";
  const file = /* 업로드할 파일 (File 객체 또는 Blob) */;
  uploadBytes(fileRef, file)
    .then((snapshot) => {
      console.log('파일 업로드 성공:', snapshot);
    })
    .catch((error) => {
      console.error('파일 업로드 에러:', error);
    });
  ```

---

### 4. getDownloadURL

```js
getDownloadURL(ref: StorageReference): Promise<string>
```

- **역할:**  
  업로드된 파일의 다운로드 URL을 생성하여 반환한다.

- **사용 예:**  

  ```js
  import { getDownloadURL } from "firebase/storage";
  getDownloadURL(fileRef)
    .then((url) => {
      console.log('파일 다운로드 URL:', url);
    })
    .catch((error) => {
      console.error('URL 생성 에러:', error);
    });
  ```

---

### 5. deleteObject

```js
deleteObject(ref: StorageReference): Promise<void>
```

- **역할:**  
  Firebase Storage의 특정 위치(즉, `ref`로 지정된 경로)에 있는 파일을 삭제한다.
  
- **사용 예:**  

  ```js
  import { deleteObject } from "firebase/storage";
  
  // 기존에 생성된 참조(fileRef)를 통해 파일 삭제
  deleteObject(fileRef)
    .then(() => {
      console.log('파일 삭제 성공');
    })
    .catch((error) => {
      console.error('파일 삭제 에러:', error);
    });
  ```

---