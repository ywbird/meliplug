---
title: "게임/시각화 툴에 대한 고민"
date: 2025-08-14T14:50:21+09:00
description: "게임/시각화 툴에 대한 고민"
toc: true
draft: true
---

## Introduction

나는 시각화를 좋아한다.
이를 공유하는 것도 좋아한다.

이것저것 만들때, 이를 공유하는 것을 염두하고 만들고 싶고, 보통 이상한 시뮬레이션을 만들기에, 컴파일 언어를 쓰고싶다.

## Criteria

 - [ ] GUI
 - [ ] Simple Draw  
  draw_rectangle(x, y, w, h) 같이 간결하게 그림 그리기 가능
 - [ ] Compile language
 - [ ] Web exportable

## Game Engines

### Love2D

Lua51 기반 게임엔진

 - [ ] GUI
 - [x] Simple Draw
  draw_rectangle(x, y, w, h) 같이 간결하게 그림 그리기 가능
 - [ ] Compile language
 - [ ] Web exportable

### Raylib

C/Rust/Go 게임 엔진

 - [ ] GUI  
  Rust에서는 포트가 되어있지 않음
 - [x] Simple Draw
  draw_rectangle(x, y, w, h) 같이 간결하게 그림 그리기 가능
 - [x] Compile language
 - [x] Web exportable

{{< callout "caution" >}}
Go에서는 Wayland 환경 비정상 작동
{{< /callout >}}

### Ebitengine

Go 기반 게임엔진

 - [x] GUI  
  <https://github.com/ebitenui/ebitenui>
 - [x] Simple Draw  
      draw_rectangle(x, y, w, h) 같이 간결하게 그림 그리기 가능
 - [x] Compile language
 - [x] Web exportable

{{< callout >}}
내가 Go를 많이 써보지 않음
{{< /callout >}}

### Macroquad

Rust 기반 게임엔진

 - [x] GUI
 - [x] Simple Draw
  draw_rectangle(x, y, w, h) 같이 간결하게 그림 그리기 가능
 - [x] Compile language
 - [x] Web exportable

{{< callout "caution" >}}
60 FPS 고정
{{< /callout >}}

### Nannou

Rust 기반 시각화 툴

 - [x] GUI
 - [x] Simple Draw
  draw_rectangle(x, y, w, h) 같이 간결하게 그림 그리기 가능
 - [x] Compile language
 - [ ] Web exportable

### WEB

JS + Canvas  /  P5.js

 - [x] GUI
 - [x] Simple Draw
  draw_rectangle(x, y, w, h) 같이 간결하게 그림 그리기 가능
 - [ ] Compile language
 - [x] Web exportable


