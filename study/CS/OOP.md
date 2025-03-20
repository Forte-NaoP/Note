# OOP

## 특징

1. 추상화
  
    대상의 공통적인 특성들을 묶어 표현하는 것 (abstract, interface class)

2. 상속

    - 기존 클래스의 속성과 메서드를 물려받는 것
    - 코드 재사용성과 확장성을 높임
    - 부모 클래스의 public, protected 멤버를 상속받는다.

3. 캡슐화

    - 데이터와 메서드를 묶고, 데이터에 대한 직접 접근을 막는 것

4. 다형성

    - 같은 이름의 메서드나 연산자가 다른 동작을 하는 것
    - 오버로딩과 오버라이딩을 통해 구현
      - 오버로딩: 이름은 같지만 시그니쳐가 다른 것
      - 오버라이딩: 상위 클래스의 메소드를 재정의 하는 것

## SOLID 원칙

1. 단일 책임 원칙 (SRP)

    클래스는 단 하나의 책임을 가지고 하나의 이유로만 변경되어야 한다.

2. 개방 폐쇄 원칙 (OCP)

    확장에는 열려 있고 수정에는 닫혀 있어야 한다.
    기존 코드를 수정하지 않고 기능을 추가할 수 있어야 한다.

3. 리스코프 치환 원칙 (LSP)

    상위 타입의 객체를 하위 타입으로 대체해도 정상적으로 동작해야 한다.  

4. 인터페이스 분리 원칙 (ISP)

    클라이언트는 자신이 사용하지 않는 메서드에 의존하지 않아야 한다.
    인터페이스를 세분화하여 클라이언트가 필요한 기능만 사용할 수 있도록 제한한다.

5. 의존성 역전 원칙 (DIP)

    고수준 모듈(비즈니스 로직 등)은 저수준 모듈(구현 세부 사항)에 의존해서는 안 된다.
    고수준, 저수준 모두 둘 다 추상화(인터페이스나 추상 클래스)에 의존해야 한다.

    DIP 위반

    ```cpp
    class EmailService {
        public:
            void SendEmail(const string& message) {
                cout << "Sending Email: " << message << endl;
            }
    };

    // 고수준 모듈: Notification은 EmailService에 직접 의존
    class Notification {
        private:
            EmailService emailService; // 저수준 모듈에 직접 의존

        public:
            void Notify(const string& message) {
                emailService.SendEmail(message); // EmailService에 직접 접근
            }
    };
    ```

    DIP 적용

    ```cpp
    class IMessageService {
        public:
            virtual void SendMessage(const std::string& message) = 0;
    };

    // 고수준 모듈
    class Notification {
        private:
            IMessageService* messageService;

        public:
            Notification(IMessageService* service) : messageService(service) {}

            void Notify(const std::string& message) {
                messageService->SendMessage(message);
            }
    };

    // 저수준 모듈
    class EmailService : public IMessageService {
        public:
            void SendMessage(const std::string& message) override {
                // 이메일 전송 로직
            }
    };
    ```
