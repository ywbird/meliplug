---
title: "구상 - Rust Blog"
description: Rust로 markdown 블로그 만들기
date: "2025-02-25T21:01:12+09:00"
tags:
 - rust
 - blog
 - typescript
 - markdown
 - web
draft: false
---

## Rust 배우는 김에 웹개발도 같이 해보자!

Rust 배우고 있는 와중, 웹개발을 접목하면서 배우려고 블로그를 만들기로 함.  
안정화(?) 될때까지는 현재 블로그 테마를 쓸 예정.

## Static Site

블로그 사이트이므로, 그냥 html로 빌드하는 사이트를 만들기로 함.  
아래 글에서 기본적인 작동 방식을 알았다.  
<https://kerkour.com/rust-static-site-generator>  

사실 개발하고싶은 방향은 fasterthanlime의 블로그이지만, Rust초보인 내가 접근하기 너무 어렵다고 판단했다.  
<https://fasterthanli.me/articles/a-new-website-for-2020>

## CMS?

일반적인 개발 블로그처럼 Markdown을 이용하기로 했다.  
Markdown을 parse하는데 이용할 툴은, [markdown-rs](https://github.com/wooorm/markdown-rs)  
  
mdast(MarkDown Abstract Syntax Tree)를 지원하므로 [remark](https://github.com/remarkjs) 플러그인을 추후에 추가할 수 있을 것 같아서 선택하였다.

:::important
좀 더 찾아본 결과, mdast가 사용하기 꽤 어렵고, html으로 다루기 어렵다는걸 알게되었다.  
pulldown-cmark를 사용해볼 계획.  
<https://github.com/pulldown-cmark/pulldown-cmark>
:::

## 기능

내가 여러 주제 (특히 수학, 공학)에 관심있는 관계로 :h[Latex]{.font-mono} 지원은 무조건 있어야 겠다고 생각.  
<https://docs.rs/tectonic/latest/tectonic/>  
Latex 엔진을 사용하기로 했다.  

tectonic은 latex를 pdf로만 compile하는 것 같으므로 추후에 개발할때는 pdf를 svg로 바꾸는 lib도 찾아봐야할듯  
<https://fasterthanli.me/series/dont-shell-out/part-1>

## Experimental

<https://github.com/typst/typst>  

markdown 보다 새로운(?) markup을 찾은듯.  
나중에 시도.

:::note
export html을 잘 지원하지 않는것 같다.  
<https://typst.app/docs/reference/html/#exporting-as-html>
pandoc으로 시도해보았지만, 여전히 latex지원이라던지 문제가 많다.
:::
