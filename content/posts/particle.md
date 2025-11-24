---
title: Particle system
description: 파티클 시스템 만들 아이디어
date: "2025-02-09T00:42:00+09:00"
tags:
 - physics
 - sdl2
 - particle
 - math
 - rust
 - idea
---

파티클 시스템 구현하기
> 폭발 만들어보고 싶음 ㅇㅇ

> 잘 된다면 물리 발표같은걸로 할 수 있을까?

> 구체화, 실현 된다면 링크 걸기

SDL2로 파티클 스트럭처 만들기
```rust
struct Particle {
	x: f32, y: f32,
	vel: Vec2
}
```

그리드 단위로 벡터로 바람? 구현하거나 파티클 타입으로 폭발하는거 처럼 보이게
```rust
struct ExplotionParticle {
	origin: Point,
	tick: f32
}

impl Particle {
	fn move(&mut self) {
		if tick < 200 { 
			self.vel += randomVectorTowardOriginorwatever;
		} else if 200 <= tick && tick < 800 {
			self.vel += randomVectorTowardUporwaever;
		} else if 800 <= tick && tick < 1300 { 
			// manipulate vector like spinning or like swirl around po=ivot point like blabahsaa
		}
	}
}
```

위같은 느낌?

아니면 진짜 폭발해서 바람이 폭발처럼 나오게 하는 방식도 가능하다면 좋을듯
수도코드:
```text
모든 그리드 벡터(바람)에 반복:
	벡터의 반대에 있는 가장 가까운 벡터에 일정? 비례? 빼기(바람으로 대기압 낮아진거 표현하려함)
	// (아니면 각 그리드에 벡터랑 압력 전부 줘도 좋고, 애초에 바람이 압력에 의한거니까 바람을 vector로 표현하는게 아니라 압력을 표현하고 getWind 같은 함수로 그때그때 바람을 측정하는것도 바쁘지 않을듯
	// 그런식으로 폴발할때 엄청난 압력을 주면(바닥에 뭔가 고체 물질 성질 추가해서 옆으로 퍼지게) 폭발하는 파티클 입자같은걸 만들 수 있지 않을까
```

이런거 표현하려고 노력하는 물리엔진이 한둘도 아닌데 내가 너무 optimistic 한걸지도?

ㅁ?ㄹ 어짜피 지금 새벽감성으로 뭔가 하고싶은거 적고있는거니까


{{< callout "note" >}}
쓰다보니 너무 길어졌다. 이건 노트가 아니라 포스트에 뻘글, 아이디어(long)같은걸 만들어야겠다.
{{</ callout >}}
