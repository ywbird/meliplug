---
title: 'Uxn/Varvara Ecosystem'
date: '2025-12-09T23:04:12+09:00'
draft: false
tags:
  - Uxn
---

{{< quote >}}
Let's try to go slow, and fix things.
{{</ quote >}}

## What is Uxn/Varvara

> Uxn/Varvara 생태계는 우리 소프트웨어의 핵심인 작은 가상 머신을 기반으로 하는 **개인 컴퓨팅 스택**으로, 이를 통해 다양한 시스템에서 동일한 애플리케이션을 실행할 수 있습니다.  
> \- [uxn](https://100r.co/site/uxn.html)

Uxn/Varvara는 [Hundred Rabbits 스튜디오](https://100r.co/site/about.html)에서 사용하는 low-level 컴퓨팅 스택이다.  
Uxntal이라는 **Assembly** 언어로 프로그래밍 가능하며, 고전 콘솔 에뮬레이터용 롬(ROM)과 비슷한 형식의 파일로 컴파일된다.

Uxn은 한마디로 CPU로써, opcodes를 통한 연산만을 지원한다.  
Uxn만을 이용해서는 평범한 cli 프로그램처럼 콘솔 입출력, 파일 접근 정도가 가능하다.  
Varvara는 기본적 스펙만 존재하는 Uxn에 키보드, 마우스, 오디오, 및 화면 같은 입출력 장치를 연결한다.  

Uxn/Varvara는 64kb의 메모리, 2개의 256bytes의 스택으로만 구성된 매우 낮은 스펙의 VM에서 동작하고, 32개의 Opcodes로 명령을 처리한다.  

{{< callout "note" >}}
Uxn은 정의(specification)이기 때문에, [Hundred Rabbits에서 기본적으로 제공하는 구현체](https://git.sr.ht/~rabbits/uxn)를 제외하고도 많은 언어들로 구현되어있다. [AwesomeUXN](https://github.com/hundredrabbits/awesome-uxn)  
{{</ callout >}}

{{< callout "note" >}}
저수준, 저스펙 에서의 구현이 목적이고, 고전 게임 같은 롬으로 작동한다는 점에서 Pico-8과 유사하기도 하다.  
<https://www.lexaloffle.com/pico-8.php>
{{</ callout >}}

{{< callout "tip" >}}
Uxn을 설치하는 것이 부담스럽다면, 웹버전으로 사용해볼 수 있다.  
[LearnUxn](https://metasyn.srht.site/learn-uxn/)
{{</ callout >}}

## Related

 - [Uxntal](/posts/uxn/uxntal/)

## References

 - <https://100r.co/site/uxn.html>
 - <https://wiki.xxiivv.com/site/uxntal.html>
 - <https://wiki.xxiivv.com/site/varvara.html>
 - <https://git.sr.ht/~rabbits/uxn>
 - <https://learnxinyminutes.com/uxntal/>
 - [(YT) The Most Bizarre and Fascinating Project I've seen! - Tsoding Daily](https://youtu.be/d41KIL4cjQM)
 - [(YT) I spent 2 days implementing Game of Life in Uxn - Tsoding Daily](https://youtu.be/rTb6NFKUmQU)

