---
date: '2025-12-08T00:11:30+09:00'
update: '2025-12-09T22:55:34+09:00'
draft: false
title: 'Uxntal'
tags:
  - Assembly
  - Uxn
toc: true
---

{{< quote >}}
A programming language for the Uxn virtual machine.
{{</ quote >}}

**Uxntal**은 **Uxn**의 언어이다.

Uxntal은 Uxn CPU 위에서 돌아가는 ROM으로 컴파일 되기 위한 Assembly 언어이다.  

{{< callout >}}
Uxntal은 여느 프로그래밍 언어와 마찬가지로 상호의존적으로 엮여진 구조이기에, 순서대로 배우기는 어렵지만, 최대한 독립적인 개념부터 정리했다.
{{</ callout >}}

## Memory

Uxn은 2개의 256bytes의 스택으로 구성된 RAM을 제공한다.  
RAM은 기본으로 작업하는 공간인 **WST**(**W**orking **ST**ack)과 추가적으로 연산에 활용할 수 있는 **RST**(**R**eturn **ST**ack)으로 이루어져있다.  

**Memory:**
 - WST(Working Stack)
 - RST(Return Stack)

## Data Structure

데이터는 0\~255(2<sup>8</sup>-1) 범위의 정수인 **byte**, 0\~65535(2<sup>16</sup>-1) 범위의 정수인 **short**로 구분되지만, 그 차이는 스택에서 차지하는 양의 차이일 뿐이다.
Byte 데이터는 두 자리 hex(`#00`)로, short 데이터는 네 자리 hex(`#0000`)로 스택에 삽입할 수 있지만, 스택 내부에서의 구분은 없다.


```tal
#00 #ab #1234
( RESULT
WST: 00 ab 12 34
)
```
{{< callout "caution" >}}
헷갈릴 수 있지만, `#00` 자체로 WST의 최상단에 `00` 바이트를 삽입하는 코드이다.  
[Literal(`LIT`)](#lit)
{{</ callout >}}

**Data:**
 - Byte
 - Short = Byte &times; 2

## Comments

`( ` 와 ` )`로 감싸진 문자열은 주석 처리된다.
(띄어쓰기도 포함이다.)
&nbsp;

```tal
( This is comment. )

( 
    This is comment too )

( ( You can also nest ) )
```


## Runes

Unxtal에는 몇 가지 char을 이용해 사용할 수 있는 기능이 있다.

### Labels

라벨은 특정 주소에 이름을 붙이는 역할을 한다. 

라벨은 **scope**와 **sublabel**으로 나뉘며, scope는 `@`으로 시작하는 문자열로 만들 수 있다. Sublabel은 `&`으로 시작하는 문자열, 또는 `@scope/name`같이 만들 수 있다. 확인할 수 있듯이, sublabel은 scope의 하위에 위치할 수 있다.

{{< columns >}}
```tal
@banana &peel
```
=
```tal
@banana @banana/peel
```
{{</ columns >}}

{{< callout >}}
Uxntal에서는 라벨로 점프할 수 있는 기능을 지원한다.  
 - [**Jump Immediate**(`JMI`)](#jmi)  
 - [**Jump Conditional Immediate**(`JCI`)](#jci)  
 - [**Jump Stash Return Immediate**(`JSI`)](#jsi)
{{</ callout >}}

{{< callout "warning" >}}
Uxntal은 hex를 일반 데이터로 받아들이기 때문에, 라벨이 `a-f`의 문자열로만 이루어 질 수 는 없다.
```tal
#00 bed INC @bed INC
( ASSEMBLY ERROR
Hexadecimal invalid: bed in @on-reset, hello.tal:2.
)
```
[`@on-reset`](#vector)
{{</ callout >}}

라벨로 다음과 같은 기능을 수행할 수 있다.


**Function**은 어느 위치에서 실행된 뒤, 다시 그 위치로 돌아오는, 말 그대로 함수이다.

```tal
@func ( -- )
  ( ... )
  JMP2r
```


**Constant**는 uxntal에 따로 변수의 문법이 없기 때문에, 특정 주소에 데이터를 쌓아두고 그 위치를 참조하여 변수로 사용한다.

```tal
@text
    "Hello 20 "World 0a
```

<span id=vector>**Vector**</span>는 일종의 **event listener**같은 개념이다.  
거의 Varvara에서만 사용되기에, 추후 Varvara를 다룰때 더 자세히 다룰 것이다.

{{< callout >}}
사실 Uxntal ROM의 시작 지점인 `0x0100`에는 기본적으로 `@on-reset`이라는 vector가 할당되어있다.
{{</ callout >}}

### Padding

Uxntal ROM은 기본적으로 `0x0100` 주소에 로드되어있고, `0x0100` 부터 작성된다. 하지만 원한다면 특정 주소부터 작성을 시작할 수 있다.

<div id=zero-page>{{< callout warning >}}
`0x0100` 이하의 주소는 **zero-page**라고 불리고, Uxn/Varvara 시스템에 의해 사용되기에, 사용자가 작성하려 하면 에러가 난다.

```tal
|00 #00
( ASSEMBLY ERROR
Writing zero-page: #00 in @on-reset, test.tal:1.
)
```
{{</ callout >}}</div>

`|`를 이용해 특정 주소부터 작성을 시작할 수 있다. 하지만 Uxn은 ROM의 `0x0100` 부터 실행할 것이기에, 실행을 원한다면 `0x0100`으로 돌려놓아야 한다. 또한, 지나온 주소에 *작성*할 수 없기 때문에 작은 순서로 작성해야한다. (라벨을 붙이는 것 만이라면 가능하다.)

{{< columns >}}
```tal
|100 #00 INC routine INC
|200 @routine
( RESULT
WST: 01
RST: 01 06
)
```

```tal
|100 #00 INC r-one INC
|200 @r-one r-two
|150 @r-two INC
( ASSEMBLY ERROR
Writing rewind: INC in @r-two, test.tal:3.
)
```
{{</ columns >}}

`$`를 이용해 현재 주소에서 상대적으로 움직일 수 있다.

라벨과 함께 이용해서 메모리의 특정 위치에 구조체(struct)를 만들 수 있다.

```tal
|d0 @player &x $2 &y $2 &health $1
(
player 구조체에 x, y가 각 short를 차지하고, health가 byte를 차지한다.
코드 내에서 player/x 등으로 호출할 수 있다. 
)
```

### Number, Ascii Runes

스택에 데이터를 삽입하는 runes

`#`로 hex값을 스택에 삽입할 수 있다. [Literal (`LIT`)](#lit)

`"`로 아스키 문자열의 각 글자를 byte로 ROM 삽입할 수 있다.

<div id=raw>{{< callout warning >}}
스택에 삽입하는 것이 아니다. 어셈블러가 `"`를 만나면, 뒤의 문자열을 아스키 hex로 변환해 **그대로 ROM**에 삽입한다.
```tal
"banana
```
```txt
$ ./uxnasm test.tal test.rom
$ xxd hello.rom
00000000: 6261 6e61 6e61                           banana
```
{{</ callout >}}</div>

### Adressing

Uxntal에는 주소를 다루는 6가지 runes가 존재한다. 각 rune은 라벨의 주소를 가져오는 역할을 한다.

**Literal\-** 은 실행 당시에 그 주소를 가져와서 **스택**에 삽입한다.

 - **Literal Relative**(`,`)는 현재 PC에서 라벨의 주소까지의 거리를 스택에 삽입한다.
 - **Literal Zero Page**(`.`)는 [zero-page](#zero-page) 라벨의 주소(byte)를 스택에 삽입한다.
 - <span id=literal-absolute>**Literal Absolute**(`;`)</span>는 라벨의 주소(short)를 스택에 삽입한다.

**Raw\-** 는 어셈블러 시점에 그 주소를 가져와서 [**ROM**에 삽입](#raw)한다.

 - **Raw Relative**(`_`)는 현재 PC에서 라벨의 주소까지의 거리를 ROM에 삽입한다.
 - **Raw Zero Page**(`-`)는 [zero-page](#zero-page) 라벨의 주소(byte)를 ROM에 삽입한다.
 - **Raw Absolute**(`=`)는 라벨의 주소(short)를 ROM에 삽입한다.

### Wrappings

 - [`( `, ` )` 괄호](#comments)는 주석이다.

 - `[`, `]` 대괄호는 그것을 포함한 내부가 무시되는 `( `, ` )` 달리, 그 자체가 무시된다. 대괄호는 가독성을 위해 opcode를 묶는 표기로 쓰인다.

 - [`{`, `}` 중괄호](#anonymous)는 anonymous, 즉 이름 없는 루틴처럼 쓰인다.

### Macro 

`%name { }`으로 매크로를 만들 수 있다.

```tal
%modulo ( num denum -- res ) {
	DIVk MUL SUB }

@routine ( -- c* )
	#18 #03 modulo JMP2r
```

## Opcodes

Uxntal은 연산자 우선순위가 없고, 그저 연산자가 프로그램된 순서대로 스택에서 실행된다.  
**PC**(**P**rogram **C**ounter)는 프로그램이 현재 opcode를 실행 직후의 메모리에서의 장소며, 각 opcode 실행 후, 2[^3]씩 증가한다.[^2]

[^3]: 2씩 증가하는 이유는 [opcode가 2 bytes를 차지](#)하기 때문이다.
[^2]: 예외로, [immediate opcodes](#immediate-opcodes)는 opcode 직후에 저장된 데이터의 길이만큼 추가된다.

```tal
#0f #06 #04 ADD MUL
( RESULT
WST: 96
)
```

1. `0x06` + `0x04` = `0x0a`
    (WST: `0f 0a`)
2. `0x0f` &times; `0x0a` =  `0x96`
    (WST: `96`)


### Notation

Uxntal은 아래와 같은 Forth 언어의 표기법을 따른다.  

`--` 앞의 각 아이템을 실행 전 스택의 상태, 뒤의 각 아이템을 실행 후 스택의 상태로 표기한다.

```tal
#12 #34 ADD
( RESULT
WST: 46
)

ADD ( a b -- c )
```

[Shorts를 대상으로 하는 연산](#2-mode)이라면, 그 아이템 뒤에 `*`을 붙여서 표기한다.

```tal
#1234 #abcd ADD2
( RESULT
WST: be 01
)

ADD2 ( a* b* -- c* )
```

[RST에 영향을 끼치는 연산](#r-mode)이라면, `.` 뒤에 그 RST의 상황을 표기한다.

{{<columns>}}
```tal
#12 STH
( RESULT
WST: 
RST: 12
)

STH ( a -- . a)
```

```tal
( BEFORE RST: 12 )
STHr
( RESULT
WST: 12
)

STHr ( . a -- a )
```
{{</columns>}} 

{{< callout >}}
{{< details summary="Stash" >}}
**Stash**(`STH`)는 스택의 최상단 byte를 소모하여 RST으로 삽입한다.  
`STHr`은 RST의 최상단 byte를 소모하여 WST으로 삽입한다.
{{</ details >}}
{{</ callout >}}

### Mode

Uxntal에는 3가지 모드가 존재한다.

| Mode                | Description         |
| -:                  | :-                  |
| **Short mode** `2`  | Short에 연산을 시행 |
| **Keep mode** `k`   | 연산 대상 값을 보존 |
| **Return mode** `r` | RST에 연산을 시행   |

<span id=2-mode>**Short mode**</span>는 byte 대신 short(두 byte)를 소모하여 연산한다.  
점프와 관련된 opcode[^1]에서는 absolute 위치를 사용한다.

[^1]: `JMP`, `JCN`, `JSR`

```tal
#0123 #abcd ADD2
( RESULT
WST: ac f0
)

ADD2 ( a* b* -- c* )
```

**Keep mode**는 연산 이후 연산에 사용된 값들을 소모하지 않고 보존한다.

```tal
#01 #23 ADDk
( RESULT
WST: 01 23 24
)

ADD2 ( a b -- a b c )
```

<span id=r-mode>**Return mode**</span>는 WST 대신 RST에서 연산을 시행한다.  

```tal
( BEFORE RST: 01 23 )
ADDr
( RESULT
RST: 24
)

ADDr ( . a b -- . c )
```

{{< callout >}}
{{< details summary="Stash" >}}
**Stash**(`STH`)는 스택의 최상단 byte를 소모하여 RST으로 삽입한다.  
`STHr`은 RST의 최상단 byte를 소모하여 WST으로 삽입한다.
{{</ details >}}
{{</ callout >}}


이러한 mode들은 혼합해 사용할 수 있다. 각 opcode는 총 8가지의 variant를 갖는다.[^4]

[^4]: [Immediate opcodes](#immediate-opcodes)는 모드를 갖지 않는다.

```tal
ADD    ( a b -- c )
ADD2   ( a* b* -- c* )
ADDk   ( a b -- a b c )
ADDr   ( . a b -- . c )
ADD2k  ( a* b* -- a* b* c* )
ADDkr  ( . a b -- . a b c )
ADD2r  ( . a* b* -- . c* )
ADD2kr ( . a* b* -- . a* b* c* )
```


따라서, Uxntal의 opcode는 각 2 bytes를 차지한다.

<table><thead>
  <tr>
    <th colspan="8">OPCODE</th>
  </tr></thead>
<tbody>
  <tr>
    <td><b>2</b></td>
    <td><b>k</b></td>
    <td><b>r</b></td>
    <td colspan="5"><b>opcode id</b></td>
  </tr>
  <tr>
    <td>0</td>
    <td>0</td>
    <td>0</td>
    <td>0</td>
    <td>0</td>
    <td>0</td>
    <td>0</td>
    <td>0</td>
  </tr>
</tbody>
</table>

### Immediate Opcodes

Opcode 중에서 일부는 스택 최상단에서 데이터를 소모하여 연산하지 않고, opcode 이후에 쓰여진 값을 ***즉시*** 가져와 사용하는 opcode들이 있다. 이들이 immediate opcodes이다.
지정된 방식으로만 작동하기 때문에 immediate opcode에는 mode가 없다.
Immediate opcodes는 그 opcode를 uxntal 소스코드에는 사용하지 않고, 추후 어셈블리가 자동으로 추가한다.

| Name                             | Opcode | Syntax     |
| -:                               | :-:    | -          |
| [Literal](#lit)                     | `LIT`  | `#`        |
| [Jump Immediate](#jmi)              | `JMI`  | `!routine` |
| [Jump Conditional Immediate](#jci)  | `JCI`  | `?routine` |
| [Jump Stash Return Immediate](#jsi) | `JSI`  | `routine`  |

<span id=lit>**Literal**</span>(`LIT`)은 `#`를 사용해 다음에 오는 데이터를 스택에 삽입한다.

```tal
#12 #abcd
( RESULT
WST: 12 ab cd
)
```

<span id=jmi>**Jummp Immediate**</span>(`JMI`)는 `!` 이후 루틴의 이름을 적는 방식으로 루틴으로 점프한다.

```tal
#00 !routine INC
@routine INC INC
( RESULT
WST: 02
)
```

위 코드에서 점프 opcode가 없었다면 결과가 `03` 이었겠지만, `@routine` 루틴(의 주소)으로 점프하였기에, 결과는 `02`이다.

{{< callout "note" >}}
`@routine`은 루틴의 해당 위치에 주소를 정의하고, 이외에 사용되는 `routine`은 그 위치의 주소를 가리킨다.

```tal
@routine @routine
( ASSEMBLY ERROR
Label duplicate: @routine in @routine, test.tal:1
)
```

{{</ callout >}}

<span id=jci>**Jummp Conditional Immediate**</span>(`JCI`)는 `?` 이후 루틴의 이름을 적는 방식으로 조건적으로 점프한다.  
연산 시, 앞의 byte를 가져와 그 값에 따라 앞의 byte를 가져와 그 값에 따라 루틴으로 점프한다. 즉, 조건 점프는 byte를 **소모**한다.

`?` 이후 루틴이 올 경우, 스택 최상위 byte를 소모하고, **`00`이 아니라면  루틴으로 점프**한다. `00`이라면 그대로 진행한다. (공식 문서에서는 PC를 2 증가시킨다고도 표현한다.)

{{< columns >}}
<div>

```tal
#00 ?routine INC @routine INC INC
( RESULT
WST: 03
)
```
최상위 byte가 `00`이므로 점프 X
</div>

<div>

```tal
#01 ?routine INC @routine INC INC
( RESULT
WST: 02
)
```
최상위 byte가 `01`이므로 점프 O</div>

{{</ columns >}}

<span id=anonymous>`?` 이후 `{`, `}` 으로 감싸진 부분</span>이 나온다면 **이전과 반대로 스택 최상위 byte가 `00`이라면** 내부를 실행한다.  

{{< columns >}}
```tal
#10 #00 ?{ INC }
( RESULT
WST: 11
)
```

```tal
#10 #01 ?{ INC }
( RESULT
WST: 10
)
```
{{</ columns >}}

이는 Uxntal에서 *헷갈리는 부분* 중 하나인데, `}`을 루틴의 정의라고 생각하고, `{`을 루틴의 라벨이라고 생각하면 그나마 나을 듯 하다.

{{< columns >}}
```tal
#10 #00 ?{ INC }
```

->

```tal
#10 #00 ?end INC @end
```
{{</ columns >}}


<span id=jsi>**Jump Stash Return Immediate**</span>(`JSI`)는 간단히 말해 루틴을 해당 위치에서 실행한다.  
`JMI`와의 차이점은 현재 PC+2의 값을 RST에 삽입하고, 루틴의 위치로 점프한다. 

```tal {linenos=inline}
#00
routine 
INC         
@routine    
( RESULT    
WST: 00     
RST: 01 05  
     ^^^^^ Line 3의 PC
)
```

{{< callout "note" >}}
{{< details summary="Jump Stash Return" >}}

**Jump Stash Return**(`JSR`)이라는 다른 opcode도 있는데, 이것은 동적으로 스택 최상위에 있는 주소로 점프함과 동시에 현재 PC+2 값을 RST에 삽입한다.  
따라서 아래 코드는 `JSI`의 예시 코드와 정확이 동일한 결과를 만든다.

```tal {linenos=inline}
#00
;routine JSR
INC         
@routine    
( RESULT    
WST: 00     
RST: 01 05  
     ^^^^^ Line 3의 PC
)
```


`JSI`가 존재 함에도 불구하고 `JSR`이 따로 존재하는 이유는, 루틴을 정해서 점프하는 `JSI`와 달리, **동적인 루틴/주소로 점프**할 수 있기 때문이다.

{{</ details >}}

[Literal Absolute(`;`)](#literal-absolute)
{{</ callout >}}

실행된 위치의 주소(PC)를 저장하기에, 루틴을 실행하고 원래의 위치로 돌아올 수 있다는 점을 이용해 함수 같이 쓰인다. 그리고 돌아오게 만들 수 있기에, 내부 어느 시점에서 쓰인다고 하여 **서브루틴**이라고도 한다.

```tal
#07 #04 modulo BRK
( 이대로 진행하면 그대로 modulo의 opcode를 실행하기 때문에, 그전에 break한다. )

( Forth notation은 코드 내에서 루틴의 sideeffect를 표기할때 주로 쓰인다. )
@modulo ( a mod -- res ) 
  DIVk MUL SUB JMP2r
( RESULT
WST: 03
)
```

{{< callout "note" >}}
{{< details summary="Break" >}}
**Break**(`BRK`)는 현재 vector의 연산을 종료한다. 이 opcode에는 모드가 없다.
{{</ details >}}
{{</ callout >}}

## Further More...

본 글에서는 Uxntal의 기본적 문법만을 설명했다. [다양한 opcode들](https://wiki.xxiivv.com/site/uxntal_reference.html)은 직접 읽어보길 바란다.

## Related

 - [Uxn/Varvara Ecosystem](/posts/uxn/uxn/)

## References

 - <https://wiki.xxiivv.com/site/uxntal.html>  
 - <https://learnxinyminutes.com/uxntal/>
