# C++ 정리

## Const

- 상수에 관한 키워드

### 일반 변수에서의 `const`

- 변수 선언시 이후 해당 변수에 대한 변경을 막는다.
- `const` 키워드 이후의 구문에 대한 변경을 막는다고 간주할 수 있다.

  ```cpp
  // 기본 자료형(int, float 등)은 무시하고 생각
  const int val = 10;
  int const val = 10;
  // -> val에 대한 변경 금지

  const int *val = ...;
  // 이후 *val = ...; 금지
  // val = ...; 가능

  const int **val = ...;
  // **val = ...; 금지
  // *val = ...; val = ...; 가능

  const int * const *val = ...;
  // **val = ...; *val = ...; 금지
  // val = ...; 가능

  ```

### 함수에서의 `const`

- 반환형, 매개변수의 `const`는 일반 변수와 동일
- 멤버 함수의 경우 함수 시그니쳐 이후에 `const`를 사용할 수 있다.
- `const` 멤버 함수의 제약조건은 다음과 같다.

  ```cpp
    class MyClass {
    private:
        int value = 10;
        mutable int mutableValue = 0;
        static int staticValue;
    public:
        int getValue() { // non-const 멤버 함수
            return value;
        }

        void SetValue(int val) const { // const 멤버 함수
            value = val; // 일반 멤버 변수 수정 불가
            staticValue = mutableValue = 1; // static, mutable 멤버 변수 수정 가능
        }

        bool IsPositive() const { // const 멤버 함수
            int val = GetValue();  // 일반 멤버 함수 호출 불가
            SetValue(0); // const 멤버 함수 호출 가능
            GetStaticValue(); // static 멤버 함수 호출 가능
            externalFunction(value); // 외부 함수 호출 가능
            return value > 0;   
        }

        static int GetStaticValue() {
            staticValue = 10; // static 멤버 함수는 static 멤버 함수/변수만 접근 가능
            return staticValue;
        }
    };

    // 외부 함수
    void externalFunction(int& val) {
        val = 50;
    }

    int MyClass::staticValue = 0; 
  ```

## 상속, 다형성, Virtual

- 상속: 부모 클래스의 속성이나 메소드를 자식 클래스가 물려 받는 것.  
  코드 재사용성 증가, 객체 모델링 유연성 증가
- 다형성: 같은 이름의 메소드나 연산자가 다른 클래스에 대해 다른 동작을 하는 것

- 타입 캐스팅
  - **업 캐스팅과 Virtual 키워드**  
    다형성을 위해 자식 클래스를 부모 클래스로 변환하는 경우

  - **다운 캐스팅과 RTTI**  
    부모 클래스의 객체를 자식 클래스로 변환하는 경우  
    `dynamic_cast`를 이용해 수행되며 런타임에 객체의 타입을 확인 후, 불가능한 경우 포인터는 nullptr 반환, 참조자는 `std::bad_cast` 예외를 던진다.

  ```cpp
  class Animal {
    public:
        virtual void says() { cout << "Animal says" << endl; }
        void what() { cout << "Animal" << endl; }
  };

    class Dog : public Animal {
    public:
        void says() override { cout << "Dog says" << endl; }
        void what() { cout << "Dog" << endl; }
    };

    Animal* a = new Animal();
    Animal* b = new Dog();

    a->says(); // Animal says
    a->what(); // Animal
    b->says(); // Dog says
    b->what(); // Animal
  ```

  - `Dog` 클래스의 인스턴스를 부모인 `Animal`로 업캐스팅했다.<br>
    함수 호출 결과를 보면 `virtual` 함수인 `says`는 원본 클래스의 함수를 잘 호출하지만 `what`은 부모 클래스의 함수만 호출된다.
  - `virtual` 함수가 있는 클래스는 컴파일 과정에서 `VTable`을 생성해 함수 호출을 관리한다.
  - `VTable`은 클래스마다 생성되며 객체는 `VTable` 포인터를 통해 자신의 클래스의 함수를 호출하게 된다.
  - non `virtual` 함수는 컴파일 타임에 정적으로 바인딩된다.
  - 가상 함수가 포함된 클래스의 메모리 레이아웃은 다음과 같다.

    ```md
    Animal 객체 메모리 레이아웃:
    +----------------------------+
    | Animal 클래스 RTTI 포인터   |
    +----------------------------+
    | Animal 클래스 VTable 포인터 | <-- a가 여길 가리키고 있음
    +----------------------------+
    | (Animal 클래스 멤버 변수)   |
    +----------------------------+
      
    Dog 객체 메모리 레이아웃:
    +----------------------------+
    | Dog 클래스 RTTI 포인터      |
    +----------------------------+
    | Dog 클래스 VTable 포인터    |  <-- b가 여길 가리키고 있음
    +----------------------------+
    | (Animal 클래스 멤버 변수)   |  
    +----------------------------+
    | (Dog 클래스 멤버 변수)      |
    +----------------------------+

    Animal VTable:
    +-----------------+
    | &Animal::speak  |  // Animal의 says() 함수 주소
    +-----------------+
      
    Dog VTable:
    +-----------------+
    | &Dog::speak     |  // Dog의 says() 함수 주소
    +-----------------+
    ```

    - 해당 메모리 레이아웃은 다음과 같은 방법으로도 확인 할 수 있다.

    ```cpp
    class A {
        private:
            int A_num;
            int A_sum;
        public:
            A() { 
                A_num = 0x12121212;
                A_sum = 0x34343434;
                printf("A 생성자\n"); 
            }
            virtual void says() { printf("A의 func\n"); }
            virtual ~A() { printf("A 소멸자\n"); }
            
    };

    class B : public A {
        private:
            int B_num;
            int B_sum;
        public:
            B() { 
                B_num = 0x56565656;
                B_sum = 0x78787878;
                printf("B 생성자\n"); 
            }
            void says() override { printf("B의 func\n"); }
            ~B() { printf("B 소멸자\n"); }
    };

    A* c = new B();
    uintptr_t* vtablePtr = *reinterpret_cast<uintptr_t**>(c); // vtable 포인터
    printf("vtablePtr: %p\n", vtablePtr);
    int* dataPtr = reinterpret_cast<int*>(reinterpret_cast<char*>(c) + sizeof(uintptr_t));
    printf("A_num: %x\n", *dataPtr);
    dataPtr = reinterpret_cast<int*>(reinterpret_cast<char*>(c) + sizeof(uintptr_t) + sizeof(int)); 
    printf("A_sum: %x\n", *dataPtr);
    dataPtr = reinterpret_cast<int*>(reinterpret_cast<char*>(c) + sizeof(uintptr_t) + sizeof(int) * 2);
    printf("B_num: %x\n", *dataPtr);
    dataPtr = reinterpret_cast<int*>(reinterpret_cast<char*>(c) + sizeof(uintptr_t) + sizeof(int) * 3);
    printf("B_sum: %x\n", *dataPtr);
    auto funcPtr = reinterpret_cast<void(*)(A*)>(vtablePtr[0]);
    funcPtr(c);

    cout << typeid(*c).name() << endl;
    const std::type_info* rttiInfo = reinterpret_cast<const std::type_info*>(vtablePtr[-1]);
    std::cout << "RTTI type name: " << rttiInfo->name() << std::endl;

    // 출력 결과
    // A 생성자
    // B 생성자
    // vtablePtr: 0x5631eb7cbcc0
    // A_num: 12121212
    // A_sum: 34343434
    // B_num: 56565656
    // B_sum: 78787878
    // B의 func
    // RTTI type name: 1B
    ```

  - 소멸자도 함수이기 때문에 소멸자를 가상 함수로 선언하지 않으면 부모 클래스의 소멸자만 호출되어 메모리 누수가 발생할 수 있다.

  - 순수 가상 함수
    - 가상 함수를 선언만 한 형태

    ```cpp
    virtual void func() = 0;
    ```

    - 순수 가상 함수가 하나라도 있는 클래스는 추상 클래스라 하며 해당 클래스는 객체 생성이 불가능하다.
    - 순수 가상 함수는 상속된 클래스에서 반드시 구현해줘야 함
    - 순수 가상 함수로만 이루어진 클래스는 인터페이스라고 함

  - 생성자 (소멸자) 내부 가상 함수 호출의 문제점
    - 부모 생성자 호출 후 자식 생성자가 호출 된다.
    - 자식 소멸자 호출 후 부모 소멸자가 호출 된다.
      - 부모 생성자 호출 시 자식은 초기화 되어 있지 않으므로 부모의 가상 함수가 호출 된다.
      - 순수 가상 함수라면 실제 구현된 함수가 존재하지 않으므로 링킹에 실패한다.
      - 소멸자에서 또한 자식 소멸자 호출이 되고 나면 이후 객체는 부모 클래스로 취급되므로 자식 클래스에서 오버라이딩 한 가상 함수가 정상적으로 호출되지 않는다.

## 포인터와 참조자

| **특징**       | **포인터**        | **참조자**                |
|----------------|------------------|---------------------------|
| **NULL 지원**  | 지원 (`nullptr`)  | 지원하지 않음              |
| **초기화**     | 나중에 초기화 가능 | 선언과 동시에 초기화 필수   |
| **대상 변경**  | 가능              | 불가능                     |
| **사용법**     | `*`, `&` 필요     | 간단 (변수처럼 사용 가능)   |
| **크기**       | 메모리 주소 크기   | 크기 없음                  |
| **산술 연산**  | 가능 (`ptr++`)    | 불가능                     | 

### 선택 기준

- **포인터를 사용할 때**:
  - 동적 메모리 할당 및 관리가 필요할 때
  - NULL 값을 가리키거나 가리키는 대상을 변경해야 할 때
  - 배열, 문자열, 하드웨어 주소를 다룰 때

- **참조자를 사용할 때**:
  - 매개변수로 데이터를 전달하여 복사를 피하고 싶을 때
  - 반환값으로 객체를 참조해야 할 때
  - 간단하고 직관적인 코드를 작성하고 싶을 때

## 객체의 초기화

  C++에서 클래스 생성자는 **객체가 생성될 때 호출되어 멤버 변수나 기타 자원을 초기화하는 특별한 함수**다.  

---

### **1. 멤버 초기화 리스트(Member Initialization List)**

- 생성자 정의에서 콜론(`:`) 뒤에 **멤버 초기화 리스트**를 사용하여 객체를 초기화
- **멤버 변수의 초기화**가 생성자의 본체가 실행되기 전에 수행되므로 효율적
- **const 멤버, 참조자 멤버**는 생성 시 초기화 되어야 하므로 초기화 리스트가 필수

```cpp
class MyClass {
private:
    int value;
    const int constValue;
    int& refValue;

public:
    MyClass(int val, int constVal, int& ref) 
        : value(val), constValue(constVal), refValue(ref) {}
};
```

---

### **2. 생성자 본문에서 초기화**

- 생성자 본문에서 직접 멤버 변수에 값을 할당하여 초기화.
- 임시 객체 생성 가능성이 있다.

```cpp
class MyClass {
private:
    int value;

public:
    MyClass(int val) {
        value = val;
    }
};
```

---

### **3. 디폴트 멤버 초기화(Default Member Initializer)**

- C++11부터 클래스 정의에서 멤버 변수를 초기화할 수 있는 **디폴트 초기화 구문**이 도입
- 멤버 변수 선언 시 초기값을 지정하며, 모든 생성자에서 동일한 초기값을 사용하려는 경우 유용함
- 초기화 리스트보다 우선 적용되지만, 명시적 초기화가 있으면 덮어쓴다.

```cpp
class MyClass {
private:
    int value = 0;
    double rate = 1.5;

public:
    MyClass() {} // 별도 초기화 없이 기본값 사용
};
```

---

### **4. 위임 생성자(Delegating Constructor)**

- C++11에서 추가된 기능으로, **생성자에서 다른 생성자를 호출**하여 멤버를 초기화
- 비슷한 초기화 로직을 여러 생성자에서 반복하지 않고 공유할 수 있다.
- 코드 중복을 줄이고, 유지보수성을 높임

```cpp
class MyClass {
private:
    int value;

public:
    // 기본 생성자
    MyClass() : MyClass(0) {} // 위임 생성자 사용

    // 파라미터를 받는 생성자
    MyClass(int val) : value(val) {}
};
```

## L-Value, R-Value, Move Semantics

- L-Value: 메모리에 존재하며 참조할 수 있는 값
- R-Value: 임시적인 값으로 참조할 수 없는 값
- Move semantics: 복사 대신 리소스의 소유권을 이동하는 개념
  - R-Value 참조(&&): R-Value를 참조할 수 있는 참조자
  - std::move: L-Value를 R-Value 캐스팅해주는 함수

  이동 생성자와 이동 대입 연산자를 구현하여 사용  
  
  복사 대신 소유권을 이전하여 기존 객체의 리소스를 재활용  
  원본 객체는 비어있는 상태가 됨

  불필요한 메모리 할당과 해제를 줄여 복사 오버헤드를 줄이고 효율적인 메모리 사용을 가능하게 한다.

  ```cpp
  class MyClass {
  private:
      int* data;
      int size;

  public:
      MyClass(int s) : size(s), data(new int[s]) { }

      // 이동 생성자
      MyClass(MyClass&& other) noexcept : size(other.size), data(other.data) {
          other.size = 0;
          other.data = nullptr;  // 소유권 이전
      }

      // 이동 대입 연산자
      MyClass& operator=(MyClass&& other) noexcept {
          if (this != &other) {
              delete[] data;  // 기존 리소스 해제
              data = other.data;
              size = other.size;
              other.data = nullptr;
              other.size = 0;
          }
          return *this;
      }

      ~MyClass() { delete[] data; }
    };

  int main() {
      MyClass obj1(10);
      MyClass obj2 = std::move(obj1);  // 이동 생성자 호출

      MyClass obj3(5);
      obj3 = std::move(obj2);  // 이동 대입 연산자 호출

      return 0;
  }
  ```

