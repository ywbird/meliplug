---
title: JPEG DCT는 어떻게 작동하는가?
description: 이미지 포멧 JPEG은 어떻게 작동할까
date: "2025-07-19T13:41:54+09:00"
draft: true
jsxgraph: true
math: true
---

## Joint Photographic Experts Group

**JPEG**

`.jpeg`, `.jpg`의 확장자로 사용되며, 무손실 포멧인 `.png`와 다르게, 손실 포멧이다.

JPEG 작동 방식

## Encoding

### 1. Downsampling

YCbCr 채널로 분리

### 2. Block splitting

8x8 청크로 분리

### 3. DCT

Doscrete Cosine Transform

1. 픽셀 값[0\~255]를 [-128\~127]로 변환
2. DCT table로 변환

$$
\begin{align}
&G_{u,v} = 
    \frac{1}{4}\alpha(u)\alpha(v)
    \sum^{7}_{x=0}\sum^{7}_{y=0}g(x,y) 
    \cos\left[\frac{(2x+1)u\pi}{16}\right] 
    \cos\left[\frac{(2y+1)v\pi}{16}\right] 
\end{align}
$$

$$
(0 \le u \lt 8, 0 \le v \lt 8)
$$

$$
\alpha(i) = \begin{cases}
&\frac{1}{\sqrt{2}} &\text{if } i = 0 \\
&1 & \text{otherwise} \\
\end{cases}
$$

$$
g(x,y) = \text{value of pixel at (x,y)}
$$

$$
\begin{equation}
G=\begin{bmatrix}
-415.38 &-30.19 &-61.20 &\cdots &0.46 \\
4.47    &-21.86 &-60.76 &\cdots &4.88 \\
-46.83  &7.37   &77.13  &\cdots &-5.65 \\
\vdots  &\vdots &\vdots &\ddots &\vdots \\
-0.17   &0.14   &-1.07  &\cdots &1.68 \\
\end{bmatrix}
\end{equation}
$$

### 4. Quantization

양자와(압축)

$$
\begin{equation}
Q=\begin{bmatrix}
 16 & 11 & 10 & 16 & 24 & 40 & 51 & 61 \\
 12 & 12 & 14 & 19 & 26 & 58 & 60 & 55 \\
 14 & 13 & 16 & 24 & 40 & 57 & 69 & 56 \\
 14 & 17 & 22 & 29 & 51 & 87 & 80 & 62 \\
 18 & 22 & 37 & 56 & 68 & 109 & 103 & 77 \\
 24 & 35 & 55 & 64 & 81 & 104 & 113 & 92 \\
 49 & 64 & 78 & 87 & 103 & 121 & 120 & 101 \\
 72 & 92 & 95 & 98 & 112 & 100 & 103 & 99
\end{bmatrix}
\end{equation}
$$

$$
B_{j,k} = \Bigl\lfloor \frac{G_{j,k}}{Q_{j,k}} \Bigr\rceil
$$

$$
\begin{equation}
B = \begin{bmatrix}
-26 &-3 &\cdots &0 \\
0   &-2 &\cdots &0 \\
\vdots &\vdots &\ddots &\vdots \\
0 &0 &\cdots &0 \\
\end{bmatrix}
\end{equation}
$$



{{< jsxgraph "jpgbox1" 500 500 >}}
<script>
(function(){
let board = JXG.JSXGraph.initBoard("jpgbox1", {
  boundingbox: [-10, 10, 10, -10],
  axis: false,
  showCopyright: false,
});

let box = [0,8];
let view = board.create('view3d', [[-4, -2], [8, 8], [box, box, [-4,4]]], {
  xPlaneRear: {visible: false},
  // xAxis: {visible: false},
  // yAxis: {visible: false},
  zAxis: {visible: false},
  yPlaneRear: {visible: false},
});

let u = board.create(
  'slider',
  [[-7, -8], [7,-8], [0,0,7]],
  {
    snapWidth: 1,
    suffixLabel: "u=",
    name: "u"
  }
)
let v = board.create(
  'slider',
  [[-7, -6], [7,-6], [0,0,7]],
  {
    snapWidth: 1,
    suffixLabel: "v=",
    name: "v"
  }
)

let f = board.jc.snippet(`
  z =
    (( u == 0 ) ? sqrt(2) : 1) *
    (( v == 0 ) ? sqrt(2) : 1) *
    cos(((2*x+1)*u*PI)/16)*
    cos(((2*y+1)*v*PI)/16)/2;
  `
  , true, 'x,y', true)
view.create('functiongraph3d', [f, box, box], {
  stepsU: 50, stepsV: 50, strokeWidth: 0.8
});
})();
</script>

<br>

<details>
  <summary>DCT whole table</summary>

{{< jsxgraph "jpgbox2" 700 700 >}}
<script>
(function(){
let board = JXG.JSXGraph.initBoard("jpgbox2", {
  boundingbox: [-10, 10, 10, -10],
  axis: false,
  showCopyright: false,
});

let box = [0,64];
let view = board.create('view3d', [[-6, -6], [12, 12], [box, box, [-4,4]]], {
  xPlaneRear: {visible: false},
  xAxis: {visible: false},
  yAxis: {visible: false},
  zAxis: {visible: false},
  yPlaneRear: {visible: false},
});


let f = board.jc.snippet(`
  z =
    (( x < 8 ) ? sqrt(2) : 1) *
    (( y < 8 ) ? sqrt(2) : 1) *
    cos(((2*x+1)*floor(x/8)*PI)/16)*
    cos(((2*y+1)*floor(y/8)*PI)/16)/4
  `, true, 'x,y', true)
view.create('functiongraph3d', [f, box, box], {
  stepsU: 100, stepsV: 100, strokeWidth: 0.8
});
})()
</script>

</details>
