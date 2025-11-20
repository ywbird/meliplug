---
title: JPEG DCT는 어떻게 작동하는가?
description: 이미지 포멧 JPEG은 어떻게 작동할까
date: "2025-11-18T11:23:44+09:00"
draft: true
jsxgraph: true
math: true
toc: true
---

{{< callout "note" >}}
본 글은 제7회 금요학술포험(2025-11-07)에서 발표한 내용을 정리한 것 입니다.  
발표 ppt와 자료는 아래 링크에 있습니다.  
 - ppt: <https://ppt.meliplug.info/ppt/jpeg>
 - viz: <https://viz.meliplug.info/jpeg-dct-web>
 - doc: <https://meliplug.info/docs/jpeg.html>
{{< /callout >}}

## Fourier Transformation

푸리에 변환이란 시간이나 공간에 대한 함수를 시간이나 공간 주파수 성분으로 분석하는 것을 말한다.

함수 $x(t)$가 복소수 범위에서 정의되어 있고 르베그 적분이 가능할 때, 이 함수의 푸리에 변환 $X(\xi)$는 다음과 같이 정의된다.
$$
\begin{align}
X(\xi) &= \int^{\infty}_{-\infty}{x(t)e^{-2\pi i \xi t}}dt \\
       &= \int^{\infty}_{-\infty}{x(t)(\cos(-2\pi \xi t)+i \sin(-2\pi \xi t))}dt \\
\end{align}
$$

## Fast Fourier Tranform

DFT(이산 푸리에 변환 Discrete Fourier Transform)은 이산적(불연속적) 입력 신호에 대한 푸리에 변환으로, 디지털 신호 분석에 주로 사용된다.  
FFT(고속 푸리에 변환)은 DFT를 빠르게 계산하는 알고리즘이다.
불연속적 값에 대한 푸리에 변환이므로, 적분이 아닌 합을 이용한다.


$$
\begin{align}
X_k &=\sum^{n-1}_{m=0}{x_me^{-2\pi ikm/n}} \\
    &=\sum^{n-1}_{m=0}{x_m(\cos(-2\pi km/n) + i\sin(-2\pi km/n))} \\
    &k = 0, ... , n-1 .
\end{align}
$$

## JPEG Encoding

### Color space transform

색 공간 변환 단계에서는 일반적으로 RGBA 인 비트맵 이미지 색공간을 YC<sub>b</sub>C<sub>r</sub>으로 변환한다.

YC<sub>b</sub>C<sub>r</sub>는 수학적으로 동일하지만 사용 목적이 다른 즉, 아날로그 기기를 위한 포멧인 YP<sub>b</sub>P<sub>r</sub>의 디지털 버전 포멧이다.

우선 YP<sub>b</sub>P<sub>r</sub>으로 변환한 뒤, YC<sub>b</sub>C<sub>r</sub>로 보정해주는 과정을 거치면 된다.

$$
\begin{align}
Y' &= K_R \cdot R' + K_G \cdot G' + K_B \cdot B' \\
P_B &= \frac{1}{2} \cdot \frac{B'-Y'}{1-K_B} \\
P_R &= \frac{1}{2} \cdot \frac{R'-Y'}{1-K_R}
\end{align}
$$
(이때 $K_R, K_G, K_B$는 모두 합하여 $1$이 되는 상수들이다.)

이후 계산된 YP<sub>b</sub>P<sub>r</sub>값을 16~235의 범위로 재조정한다.


### Downsampling

인간이 색을 인식하는 방법에 명도가 가장 중요하고, 색채는 명도에 비해 중요하지 않기 때문에, YC<sub>b</sub>C<sub>r</sub>로 인해 명도가 분리된 상황에서 색채를 나타내는 값을 뭉개버리는 방식이다.
YUV라고 불린다.

JPEG에서는 보통 4:2:0 이라 불리는 4x4 구역에서 각 2x2 구역을 좌상단 색으로 뭉개는 방식을 사용한다.

### Blockspliting

DCT가 짧은 범위 내에서 작동하도록 이미지를 8x8 크기의 블럭들로 나누는 단계이다.

### Discrete cosine transform

푸리에 변환이 복소평면에서 원의 무게중심을 적분하는 방식이었지만, JPEG 테이블처럼 짝함수(우함수)를 구할 것이 확실한 상황에서 짝함수만을 얻기 위해 홀함수인 Sine을 무시하고, Cosine만을 얻기 위해 이산연산을 한다.

$$
\begin{align}
X_k &=\sum^{N-1}_{n=0}{x_n(\cos(-2\pi kn/N) + i\sin(-2\pi kn/N))} \\
    &k = 0, ... , N-1 .
\end{align}
$$

$$
\downarrow \\
$$

$$
X_k = \sum^{N-1}_{n=0}{x_n \cos\left[\frac{(2n+1)k\pi}{2N}\right]},\quad k=0,...N-1
$$

위 이산 코사인 변환을 2차원으로 적용하면 아래와 같다.

$$
G_{u,v} = \frac14\alpha(u)\alpha(v)\sum^7_{x=0}\sum^7_{y=0}g_{x,y}\cos\left[\frac{(2x+1)u\pi}{2\times8}\right]\cos\left[\frac{(2y+1)v\pi}{2\times8}\right]
$$

 - $u$: 수평 DCT, $0\leq u<8,\,u\in\mathbb{Z}$
 - $v$: 수직 DCT, $0\leq v<8,\,v\in\mathbb{Z}$
 - 정규화를 위한 상수: $\alpha(i) = \begin{cases}
    \frac{1}{\sqrt{2}}, \quad &(i=0)\\
    1, \quad &(i>1)
    \end{cases}$
 - $g_{x,y}$: $(x,y)$에서 픽셀 값
 - $G_{u,v}$: $(u,v)$에서 DCT 계수 값

### Quantization

인간의 시각은 넓은 영역에서의 미세한 밝기 차이는 구별할 수 있지만, 고주파의 밝기 변화를 감지하는데 둔하다.

아래 양자화 계수로 DCT 계수를 나누어 반올림하면 그에 맞춰 작은 값들은 0에 가까운 값을 사용하게 되어 적은 용량을 차지하게 된다.

$$
Q = \begin{bmatrix}
&16 &11 &10 &16 &24 &40 &51 &61 \\
&12 &12 &14 &19 &26 &58 &60 &55 \\
&14 &13 &16 &24 &40 &57 &69 &56 \\
&14 &17 &22 &29 &51 &87 &80 &62 \\
&18 &22 &37 &56 &68 &109 &103 &77 \\
&24 &35 &55 &64 &81 &104 &113 &92 \\
&49 &64 &78 &87 &103 &121 &120 &101 \\
&72 &92 &95 &98 &112 &100 &103 &99 \\
\end{bmatrix}
$$

### Entropy coding

Quantization(양자화)의 결과, DCT 계수들은 좌상단에서 우하단으로 내려갈수록 작아지는 형태가 된다.

압축의 핵심은 반복되는 데이터를 제거(생략)하는 것인데, 위의 형태를 알고, 좌상단에서 우상단으로 지그재그로 내려가면서 배열하면 0에 가까운 값들이 뭉치게 되어 압축에 용이해진다.

## References

 - <https://en.wikipedia.org/wiki/Riemann_sum>
 - <https://ko.wikipedia.org/wiki/리만_합>
 - <https://en.wikipedia.org/wiki/Fourier_transform>
 - <https://ko.wikipedia.org/wiki/푸리에_변환>
 - <https://en.wikipedia.org/wiki/Fast_Fourier_transform>
 - <https://ko.wikipedia.org/wiki/고속_푸리에_변환>
 - <https://en.wikipedia.org/wiki/Discrete_Fourier_transform>
 - <https://ko.wikipedia.org/wiki/이산_푸리에_변환>
 - <https://en.wikipedia.org/wiki/Discrete_cosine_transform>
 - <https://en.wikipedia.org/wiki/JPEG>
 - <https://en.wikipedia.org/wiki/Y′UV>
 - <https://namu.wiki/w/YUV>
