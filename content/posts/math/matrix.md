---
title: Matrix 배우기
description: Matrix 행렬 배우기
math: true
draft: true
---

ANN(Artificial Neural Network)를 배우기 위해 행렬 연산을 배울 필요가 있었다.

## Matrix, 행렬

수학에서 행렬은 수를 직사각형 모양으로 배열한것이다.

> 나는 아래의 방식으로 이해했다.  
> 스칼라 $\RA$ 벡터 $\RA$ 행렬  
> <br>
> $
 123456 \RA (1,2,3,4,5,6) \RA
 \begin{bmatrix}
 1 & 2 & 3 \\
 4 & 5 & 6 \\
 \end{bmatrix}
 $

$$
A = \begin{bmatrix}
a_{11} & a_{21} & a_{31} \\
a_{12} & a_{22} & a_{32} \\
\end{bmatrix},
\quad
A_{1,2} = a_{12}
$$

### 특이 Matrix


<table><tbody>
  <tr>
    <td>Row Matrix</td>
    <td>Column Matrix</td>
    <td>Square Matrix</td>
  </tr>
  <tr>
    <td>
        $$
        \begin{bmatrix}
        4 & 5 & 6 \\
        \end{bmatrix}
        $$
    </td>
    <td>
        $$
        \begin{bmatrix}
        4 \\
        5 \\
        6 \\
        \end{bmatrix}
        $$
    </td>
    <td>
        $$
        \begin{bmatrix}
        1 & 2 & 3 \\
        4 & 5 & 6 \\
        7 & 8 & 9 \\
        \end{bmatrix}
        $$
    </td>
  </tr>
</tbody>
</table>

## 연산

$$
A = \begin{bmatrix}
a_{11} & a_{21} & a_{31} \\
a_{12} & a_{22} & a_{32} \\
\end{bmatrix},
\quad
B = \begin{bmatrix}
b_{11} & b_{21} & b_{31} \\
b_{12} & b_{22} & b_{32} \\
\end{bmatrix}
$$

### 덧셈/뺄셈

행렬의 크기가 같아야만 할 수 있다.

$$
A\pm B=\begin{bmatrix}
a_{11}\pm b_{11} & a_{21}\pm b_{21} & a_{31}\pm b_{31} \\
a_{12}\pm b_{12} & a_{22}\pm b_{22} & a_{32}\pm b_{32} \\
\end{bmatrix}
$$

### 실수배

$$
A = \begin{bmatrix}
a_{11} & a_{21} & a_{31} \\
a_{12} & a_{22} & a_{32} \\
\end{bmatrix},
\quad
c\cdot A = \begin{bmatrix}
c\cdot a_{11} & c\cdot a_{21} & c\cdot a_{31} \\
c\cdot a_{12} & c\cdot a_{22} & c\cdot a_{32} \\
\end{bmatrix}
$$

### 곱셈


