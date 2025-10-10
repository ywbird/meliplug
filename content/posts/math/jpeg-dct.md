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
    cos(((2*y+1)*v*PI)/16)/4;
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
