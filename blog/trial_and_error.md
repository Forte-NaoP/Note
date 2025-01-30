# Github Pages 시행착오들

1. 링크는 `https://` 로 시작해야 `htmlproofer` 검사에서 오류가 나지 않음
2. markdown 문서에서 상대경로로 포함된 리소스는 절대 경로로 변환해줘야 함.

    ```html
    {% assign remote_url = "https://raw.githubusercontent.com/Forte-NaoP/Note/main/study/Unreal/lecture.md" %}
    {% assign img_url = remote_url | remove: "lecture.md" | append: "capture/" %}

    {% capture remote_content %} <!--remote_content 변수에 remote resource 저장 -->
        {% remote_include https://raw.githubusercontent.com/Forte-NaoP/Note/main/study/Unreal/lecture.md %}
    {% endcapture %}

    {{ remote_content  <!--remote_content에서 상대경로로 포함된 리소스를 절대 경로로 변환 -->
    | replace: './capture/', img_url 
    }}
    ```