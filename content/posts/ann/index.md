---
title: ANN 구현하기 - 1. NEAT 구현하기
description: ANN을 구현하기
draft: true
---

## ANN 방식 고르기

ANN(Artificial Neural Network)에도 여러 방식이 있다.  
[WIKIPEDIA - Types of artificial neural networks](https://en.wikipedia.org/wiki/Types_of_artificial_neural_networks)

그중 사용할 것은 NEAT(**N**euro**E**volution of **A**ugmented **T**opologies)

NEAT는 다음 세 단계를 거친다.

### 평가

전 단계에서 만들어진 ANN을 사용해 시뮬레이션을 실행하고 결과를 평가한다.

### 선택

평가 결과중 상위 20%-30%를 제외하고 제거한다.

### 변형

다음 세가지 변형 중 하나를 실행한다.

 - 연결 변화  
    존재하는 연결 중 하나의 weight를 변화시킨다.
 - 연결 추가  
    새로 연결을 추가한다.
 - 노드 추가  
    존재하는 연결 중 하나를 나눠서 사이에 노드를 삽입한다.

## 구현

연결, 노드, neat개체를 정의한다.
```go
type NodeKind int

const (
	Input NodeKind = iota
	Output
	Hidden
	Bias
)

type Conn struct {
	From   int
	To     int
	Weight float64
}

type Node struct {
	Idx   int
	Kind  NodeKind
	Value float64
}

type NeatEntity struct {
	Nodes []Node
	Conns []Conn
	ins   []int
	outs  []int
	input []float64
}
```

### 노드 정렬 - Kahn's algorithm

노드의 값를 계산할때, 이미 계산되지 않은 노드를 참조하면 안되기 때문에, 들어오는 연결이 없는 노드를 우선으로 정렬해야한다.

Topological Sorting을 이용해 노드를 정렬한다.  
[WIKIPEDIA - Topological sorting](https://en.wikipedia.org/wiki/Topological_sorting)

<pre>
<i>L</i> ← Empty list that will contain the sorted elements
<i>S</i> ← Set of all nodes with no incoming edge

<b>while</b> <i>S</i> <b>is not</b> empty <b>do</b>
    remove a node <i>n</i> from <i>S</i>
    add <i>n</i> to <i>L</i>
    <b>for each</b> node <i>m</i> with an edge <i>e</i> from <i>n</i> to <i>m</i> <b>do</b>
        remove edge <i>e</i> from the <i>graph</i>
        <b>if</b> <i>m</i> has no other incoming edges <b>then</b>
            insert <i>m</i> into <i>S</i>

<b>if</b> <i>graph</i> has edges <b>then</b>
    <b>return</b> error   (graph has at least one cycle)
<b>else</b> 
    <b>return</b> <i>L</i>   (a topologically sorted order)
</pre>


