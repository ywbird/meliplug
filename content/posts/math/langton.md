---
title: "Langton's Ant"
date: 2025-07-30T03:06:11+09:00
description: "Langton's Ant에 대한 탐구"
---

Cellular Automaton의 한 종류로, 개미가 보드 위에서 움직이는 모습을 구현 한 듯 한 모양새.

무한한 2차원 그리드 보드에서 개미가 이동한다.

기본적인 룰.
 - 검은색 셀에서 90° 시계 방향으로 회전하고, 현재 셀의 색을 반전시키고, 한 칸 앞으로 이동한다.
 - 흰색 셀에서 90° 시계 반대 방향으로 회전하고, 현재 셀의 색을 반전시키고, 한 칸 앞으로 이동한다.

<table><thead>
  <tr>
    <th colspan="4">Current Color</th>
  </tr></thead>
<tbody>
  <tr>
    <td colspan="2">0</td>
    <td colspan="2">1</td>
  </tr>
  <tr>
    <td>Write</td>
    <td>Rotation</td>
    <td>Write</td>
    <td>Rotation</td>
  </tr>
  <tr>
    <td>1</td>
    <td>R</td>
    <td>0</td>
    <td>L</td>
  </tr>
</tbody>
</table>

# Turmites

'termites' 흰개미

튜링(Turing) 개미 라는 의미에서 'turing' + 'termites' = 'turmites'가 되었다.

기존의 langton's ant에서 개미에게 상태와 추가적 색을 부여하므로써 더욱 다양한 형태를 그릴 수 있다.



# Turing Complete

Langton's Ant는 튜링 완전 머신이다.

(간략화를 위해 `state n`을 `Sn`, `color n`을 `Cn`으로 부르자.)

더욱 복잡하게 만들 수 도 있지만, 간단하게 if문을 만들어보자면 다음과 같다.

<table class="center"><thead>
  <tr>
    <th colspan="2" rowspan="3"></th>
    <th colspan="9">Current Color</th>
  </tr>
  <tr>
    <th colspan="3">C0</th>
    <th colspan="3">C1</th>
    <th colspan="3">C2</th>
  </tr>
  <tr>
    <th>Write</th>
    <th>Rotation</th>
    <th>Next State</th>
    <th>Write</th>
    <th>Rotation</th>
    <th>Next State</th>
    <th>Write</th>
    <th>Rotation</th>
    <th>Next State</th>
  </tr></thead>
<tbody>
  <tr>
    <td style="font-weight: bold;" rowspan="2">Current State</td>
    <td style="font-weight: bold;">S0</td>
    <td></td>
    <td>X</td>
    <td>1</td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td style="font-weight: bold;">S1</td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td>R</td>
    <td></td>
    <td></td>
    <td>L</td>
    <td></td>
  </tr>
</tbody>
</table>


 - `S0` 상태에서 `C0`에 가면 판별 전용 상태엔 `S1`로 변화.
 - `S1` 상태에서 다음 셀이 `C1`이면 오른쪽으로 회전, `C2`이면 왼쪽으로 회전


