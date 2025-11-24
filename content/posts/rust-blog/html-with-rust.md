---
title: "Rust로 HTML 사용하기 - Rust Blog"
date: "2025-02-27T03:35:04+09:00"
description: Rust로 html 관리, 조작하기 
draft: false
toc: true
---

## 왜?

블로그 사이트를 만드려면 여러 블로그 글을 html 파일로 만들어야 하는데, 이 페이지들이 하나의 레이아웃(header, footer, etc.)으로 통일되어야 하므로, 블로그 글을 미리 만들어진 템플릿에 삽입하는 것이 일반적이다.  

Rust가 아닌 ReactJS, Svelte, Vue 등은 웹을 기반으로 상정하고 만들어졌기에 기본적으로 이러한 템플릿 기능을 지원 하지만, Rust는 앞선 경우와 다르기에(웹을 상정하고 만들어지지 않았기에) 템플릿 엔진을 사용해야한다.

### 간단한 템플릿

가장 기본적인 템플릿 엔진은 아래와 같이 만들 수 있겠다.

```rust title="main.rs" /[ >]+({})/

fn render(title: String, content: String) -> String {
  format!(r#"
  <!DOCTYPE html>
  <html>
    <head>
      <title>{}</title>
    </head>
    <body>
      {}
    </body>
  </html>
  "#, title, content)
}


fn main() {
    let page = render("Home".to_string(), "<p>Hello world</p>".to_string());
    
    println!("{}", page);
}
```

{{< details summary="🔎 Result" >}}
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Home</title>
  </head>
  <body>
    <p>Hello world</p>
  </body>
</html>
```
{{</ details >}}

나는 이보다 더 전문적(?)인 라이브러리를 쓰기로 했다.

Rust 웹 템플릿 엔진을 구현한 방법에는 여러가지가 있겠지만, 그중 두개의 Crate를 골라보았다.

## Maud

<https://maud.lambda.xyz>  

매크로를 이용해 structure 처럼, JSON 처럼 html을 작성할 수 있다.


```rust title="src/main.rs"
use maud::html;

fn main() {
    let name = "ywbird";
    let markup = html! {
        p { "Hi, " (name) "!" }
    };
    println!("{}", markup.into_string());
}
```
```html
<p>Hi, ywbird!</p>
```


### Attributes

maud의 특이한 점은 css selector로 class, id를 정의할 수 있다는점

```rust ".container"
use maud::html;
html! {
	div.container { ... }
};
```
```html
<div class="container"> ... </div>
```

이외의 attrubutes는 다음과 같이 정의할 수 있다.

```rust "href"
use maud::html;
html! {
	a href="https://example.com" { "Link" }
};
```
```html
<a href="https://example.com">Link</a>
```

### Children

다른 maud 객체를 삽입하는 것이 가능하다.

```rust "content"
use maud::html;
let content = html! { p { "Hello world!" } };
html! {
	section { 
		(content)
	}
}
```
```html
<section>
	<p>Hello world!</p>
</section>
```


---

## Askama

<https://github.com/rinja-rs/askama>

Django의 템플릿 엔진에서 영감을 받아 만들어졌다고 한다.  
Template에 들어갈 변수의 타입을 미리 선언할 수 있다는게 장점.  
:h[(maud로도 function에 type을 선언하면 가능할 듯하다.)]{.sm}

```html title="hello.html"
<p>Hello, {{ name }}!</p>
```

```rust title="src/main.rs"
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

fn main() {
    let hello = HelloTemplate { name: "world" };
    println!("{}", hello.render().unwrap());
}
```
```html
<p>Hello, world!</p>
```

{{< callout "note" >}}
물론,inline에서 템플릿 정의도 가능하다.

{{< details summary="🗒️ Code" >}}
```rust title="src/main.rs"
use askama::Template;

#[derive(Template)]
#[template(source = r#"
<p>Hello, {{ name }}!</p>
"#)]
struct HelloTemplate<'a> {
    name: &'a str,
 }

fn main() {
    let hello = HelloTemplate { name: "world" };
    println!("{}", hello.render().unwrap());
}
```
```html
<p>Hello, world!</p>
```  
{{</ details >}}
{{</ callout >}}

---

## Raw Render

기본적으로 이러한 템플릿 엔진들은 raw html을 렌더하지 않는다.  
아래처럼 말이다.

```rust del="<" del=">"
use maud::html;
let body = html! {
	"<script>alert('hello')</script>"
}
```
```html ins="\&lt;" ins="\&gt;"
&lt;script&gt;alert('hello')&lt;/script&gt;
```

각 엔진은 다양한 방법으로 이를 해결한다.

### Maud

```rust "PreEscaped"
use maud::PreEscaped;

html! {
	(PreEscaped("<script>alert('hello')</script>"))
}
```
```html
<script>alert('hello')</script>
```

### Askama

```rust "safe"
use askama::Template;

#[derive(Template)]
#[template(source = r#"{{ content|safe }}"#)]
struct Layout {
    content: String,
}

Layout { content: "<script>alert('hello')</script>" }
```
```html
<script>alert('hello')</script>
```

---

이처럼 직접 html을 넣는것은 위험할 수 있기에, 직접 렌더한 데이터만 넣어야할 것 같다.

## 결론

블로그 만들때 CSS, JS 같은 import도 많고, 여러 html을 관리할 것 같기 때문에, html syntax를 지원하는 Askama를 사용하기로 했다.  

