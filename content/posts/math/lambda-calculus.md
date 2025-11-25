---
title: Lambda Calculus
description: 기반적인 함수적 이론
math: true
date: 2025-10-05T15:52:14+09:00
tags:
    - Math
    - 금요학술포럼
---

**기반적인 함수적 이론**  
아래의 <u>**문법과 연산만으로**</u> 모든 수학적, 컴퓨터적 연산을 구현할 수 있다.

{{< columns class="same-size" >}}

{{< fieldset "문법" >}}

1. **Variable**  - 변수
2. **Abstraction** - 추상화
3. **Abstraction** - 적용

{{< /fieldset >}}

{{< fieldset "연산" >}}

1. **$\alpha$-conversion($\alpha$-변환)**
2. **$\beta$-reduction($\beta$-축약)**

{{< /fieldset >}}

{{< /columns >}}

### 문법

1. $x$ : **변수**, 함수에서 쓰이는 매계변수를 표현
2. $(\lambda x . M[x])$ : **람다 추상화**, 함수를 정의. $x$를 변수로 받아서 $M[x]$을 반환  
    ($M[x]$: $x$를 포함하는 식 $M$)

3. $(\lambda x . M[x]) N$ : **적용**, $M$에 $N$을 적용 (이후 $\beta$-축약으로 이용)

### 연산

1. $(\lambda x.M[x]) \longrightarrow (\lambda y.M[y])$ : **$\alpha$-conversion($\alpha$-변환)**,  
    이때 $(\lambda x.M[x])$, $(\lambda y.M[y])$를 **$\alpha$-equivalent($\alpha$-동치)** 라고 함

2. $(\lambda x.M[x])N \longrightarrow_{\beta} M[x:=N]$ : **$\beta$-reduction($\beta$-축약)**, $M$의 $x$를 $N$으로 대체

## References

 - <https://ywbird.github.io/ppt.old/lambda-ppt>
