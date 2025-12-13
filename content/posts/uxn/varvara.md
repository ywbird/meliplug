---
draft: true
title: 'Varvara'
tags:
  - Assembly
  - Uxn
---

{{< quote >}}
Uxn is to Varvara, what the 6502 is to the Nintendo.
{{</ quote >}}

**Varvara**는 **Uxn**의 시스템이다.

Varvara는 Uxn이 외부 세계와 소통하기 위한 인터페이스이다.

출력은 zero-page의 주소로 데이터를 보내면 그걸 Varvara가 이용하여 출력하는 방식이다.
입력은 마찬가지로 zero-page의 주소에서 Varvara가 설정 해놓은 데이터를 가져와서 사용하는 방식이다.

예를 들어 ascii 문자를 출력하는 것은 다음과 같다.  
Ascii `A`값 `0x41`을 주소 `0x18` 에 작성하는 것이다.


```tal
#41 #18 DEO ( 'A'의 ascii 값은 65=0x41 )
( OUTPUT
A
)
```

{{< callout >}}
**Device Output**(`DEO`)은 스택에서 값을 가져와서 주어진 zero-page의 장치 부분에 작성한다.
```tal
DEO ( val device8 -- )
```
https://wiki.xxiivv.com/site/uxntal_reference.html#deo
{{</ callout >}}

{{< callout >}}
주소 `0x18`은 zero-page에서 보통 `@Console/write`로 정의되는 주소이다.
https://wiki.xxiivv.com/site/varvara.html#console
{{</ callout >}}

Zero-page의 주소도 당연하게 라벨을 부여하는 것이 가능하다.


```tal
|10 @Console/vector $2 &read $5 &type $1 &write $1 &error $1
#41 .Console/write DEO
( OUTPUT
A
)
```
