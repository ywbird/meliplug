---
title: "Rust로 Markdown 사용하기 - Rust Blog"
date: "2025-03-03T10:24:59+09:00"
description: Rust로 markdown 관리, 조작하기 
draft: false
toc: false
---

## Markdown

사실 모두 알고 있으리라 생각하지만...  

[Markdown](https://en.wikipedia.org/wiki/Markdown)은 [Daring Fireball](https://daringfireball.net/)을 운영하고있는 John Gruber가 만든 markup 언어이다.  
<https://daringfireball.net/projects/markdown/>

Markdown은 '원문을 읽기 쉽도록' 만들어졌다.

:h[디테일한 설명은 질리도록 들어왔으리라 생각한다...]{.sm}

## Rust Markdown?

사실 markdown은 웹에서 엄청나게 많이 쓰이기에 JS로 이루어진 compiler가 많지만, Rust로 작성된것도 꽤 있다.

 - [wooorm/markdown-rs](https://github.com/wooorm/markdown-rs)
 - [pulldown-cmark/pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark)


두 crate를 모두 사용해본 결과, `mardown-rs`는 간단한 compile에는 사용할 수 있지만, :h[md]{.font-mono.red}:h[ast]{.font-mono}(**M**ark**d**own **A**bstract **S**yntax **T**ree)를 지원함에도 불구, 훨신 복잡한 plugin 개발 등에는 적합하지 않다고 판단했다.

## pulldown-cmark

오히려 mdast를 지원하지 않는 `pulldown-cmark`가 더 많은 cusomization을 할 수 있었다.  

`pulldown-cmark`는 CommonMark를 기본적으로 지원하고, 추가 기능을 켜고 끌 수 있다.  
`pulldown-cmark`는 기본적으로 꽤 많은 추가 기능을 지원한다.  

 - gfm
 - heading attributes
 - yaml metadata
 - math
 - and more...

:::note
Every Options  
[pulldown_cmark - Rust::Options](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/struct.Options.html)
:::

---

간단하게는, 아래와 같이 `Parser`를 이용할 수 있다.  

```rust
let raw_markdown = "hello world";
let parser = pulldown_cmark::Parser::new(raw_markdown);

let mut html_output = String::new();
pulldown_cmark::html::push_html(&mut html_output, parser);
assert_eq!(&html_output, "<p>hello world</p>\n");
```

신기한 점은, `Parser` 객체가 `iterator`라는 점이다.  

`Parser`객체를 `FnMut(Event<'_>) -> Event<'_>` closure로 돌면, 간단히 플러그인을 만들 수 있었다.  

복잡해 보이지만, `Parser`를 map하면, `Event`객체가 반환되고, 이를 다시 `Event`객체를 반환하면 된다는 뜻.  

### Frontmatter Extract

frontmatter를 추출하는 plugin을 간단히 만들 수 있다.

```rust
let raw_markdown = r#"
---
title: "pulldown cmark frontmatter"
description: "frontmatter plugin"
---
"#

    let options = {
	let mut opt = Options::empty();
	opt.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
	opt
    };

let mut raw_frontmatter = String::new();
let mut frontmatter_started = false;

let parser = Parser::new_ext(&raw_markdown, options)
    .map(|event| {
	match event {
	    Event::Start(Tag::MetadataBlock(_)) => {
		frontmatter_started = true;
	    },
	    Event::End(TagEnd::MetadataBlock(_)) => {
		frontmatter_started = false;
	    },
	    Event::Text(text) => {
		if frontmatter_started {
		    let _ = &raw_frontmatter.push_str(&text);
		}
	    },
	    _ => ()
	}
	
	event
    })
    
    assert_eq!(
	&raw_markdown,
	r#"title: "pulldown cmark frontmatter"
description: "frontmatter plugin""#);
```

특정 태그가 열리고 닫히는 `Event`에 따라, frontmatter를 추출할 수 있다.

### Plugin

위 예시처럼, 태그가 열리고 닫히는 순간 사이에 있는 내용을 얻어서 직접 플러그인을 만들 수 있는다.  
하지만, 위와같이 모든 플러그인을 작성하면, 플러그인 하나마다, 거진 하나 또는 그 이상의 `mut` 변수가 추가된다는 뜻이다.  

그래서, 하나의 플러그인을 하나의 객체로 만들기로 했다.

:::note
나도 Rust 초보이기 때문에, 이게 맞는 접근인지 잘 모른다는 점...
:::

`mut` 변수가 없는 플러그인은 다음과 같이 간단히 만들 수 있다.  

#### 예시 : KATEX 렌더 플러그인

```rust title="plugins.rs"
pub struct MathPlugin {}

impl MathPlugin {
    pub fn new() -> Self { Self {} }
    pub fn apply (&self) -> impl FnMut(Event<'_>) -> Event<'_> {
	return |event| { // code
	    match event {
		Event::InlineMath(text) => {
		    let opts = katex::Opts::builder()
			.display_mode(false)
			.trust(true)
			.output_type(OutputType::Mathml)
			.build().unwrap();
		    let html = katex::render_with_opts(&text, &opts).unwrap();
		    Event::Html(html.into())
		},
		Event::DisplayMath(text) => {
		    let opts = katex::Opts::builder()
			.display_mode(true)
			.trust(true)
			.output_type(OutputType::Mathml)
			.build().unwrap();
		    let html = katex::render_with_opts(&text, &opts).unwrap();
		    Event::Html(html.into())
		}, 
		_ => event
	    }
	}
    }
}
```

```rust title="main.rs"
let math_plugin = MathPlugin::new();

let parser = Parser::new_ext(&raw_markdown, options)
    .map(math_plugin.apply());
```

---

#### CodeBlock Plugin

Math의 경우에는 내용물이 그대로 `Event`의 내용으로 반환되지만, Codeblock은 그렇지 않고, 여닫는 `Event`만 나오기에 앞선 frontmatter과 비슷한 접근을 해야했다.

이 경우, `struct`에 item을 사용해보았다.

:::note
syntax highlight에는 `syntect`라는 크레이트를 사용해보았다.  
추후 가능하면 `tree-sitter-highlight`로 바꿔보고싶.
:::

```rust
pub struct CodeHighlightPlugin {
    lang: String,
    source: String,
    is_in: bool,
}

impl CodeHighlightPlugin {
    pub fn new() -> Self {
	Self {
	    lang: Default::default(),
	    source: Default::default(),
	    is_in: false
	}
    }
    pub fn apply(&mut self) -> impl FnMut(Event<'_>) -> Event<'_> {
	return |event| {
	    match &event {
		Event::Text(text) => {
		    if self.is_in {
			self.source.push_str(text);
			return Event::Text("".into());
		    }
		},
		Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
		    self.lang = lang.to_string();
		    self.source = Default::default();
		    self.is_in = true;
		    return Event::Text("".into());
		},
		Event::End(TagEnd::CodeBlock) => {
		    self.is_in = false;
			
			let html = highlight_codeblock(self.source);
			
		    return match html {
			Ok(out) => Event::Html(out.into()),
			Err(err)=> Event::Html(format!("<pre>Highlight Error: {:?}</pre>", err).into()),
		    };
		},
		_ => ()
	    }

	    event
	}
    }
}
```

컴파일러가 다음 비명을 지르며 에러와 해결책을 알려주었다.

```ansi
[22;1;91merror[E0700][39m: hidden type for `impl for<'a> FnMut(pulldown_cmark::Event<'a>) -> for<'a> pulldown_cmark::Event<'a>` captures lifetime that does not appear in bounds
[22m  [22;1;94m--> [22;39msrc/plugins.rs:61:9
   [22;1;94m|
60[22;39m [22;1;94m|[22;39m       pub fn apply(&mut self) -> impl FnMut(Event<'_>) -> Event<'_> {
   [22;1;94m|[22;39m                    [22;1;94m---------[22;39m     [22;1;94m----------------------------------[22;39m [22;1;94mopaque type defined here
[22;39m   [22;1;94m|[22;39m                    [22;1;94m|
[22;39m   [22;1;94m|[22;39m                    [22;1;94mhidden type `{closure@src/plugins.rs:61:9: 61:16}` captures the anonymous lifetime defined here
61[22;39m [22;1;94m|[22;39m       return |event| {
   [22;1;94m|[22;39m [22;1;91m ____________^
[94m62[22;39m [22;1;94m|[22;39m [22;1;91m|[22;39m         match &event {
[22;1;94m63[22;39m [22;1;94m|[22;39m [22;1;91m|[22;39m         Event::Text(text) => {
[22;1;94m64[22;39m [22;1;94m|[22;39m [22;1;91m|[22;39m             if self.is_in {
[22;1;94m...[22;39m  [22;1;91m|
[94m95[22;39m [22;1;94m|[22;39m [22;1;91m|[22;39m         event
[22;1;94m96[22;39m [22;1;94m|[22;39m [22;1;91m|[22;39m     }
   [22;1;94m|[22;39m [22;1;91m|_____^
[22;39m   [22;1;94m|
[96mhelp[22;39m: add a `use<...>` bound to explicitly capture `'_`
   [22;1;94m|
60[22;39m [22;1;94m| [22;39m    pub fn apply(&mut self) -> impl FnMut(Event<'_>) -> Event<'_>[92m + use<'_>[39m {
   [22;1;94m|[22;39m                                                                   [92m+++++++++
[m
```

그래서 컴파일러의 명령대로 `+ use<'_>`를 추가했더니, 잘 작동하였다.

```rust ins="+ use<'_>"
...
	}
    }
    pub fn apply(&mut self) -> impl FnMut(Event<'_>) -> Event<'_> + use<'_> {
	    return |event| {
	        match &event {
...
```

Rust Lifetime Specifier 작동을 잘 몰라서 정확히는 모르겠지만, 아마 `&mut self`가 추가되었기에 이를 다룰 것이 필요했던 모양.

## Next

다음에는 이 포스팅에도 쓰고있는 Directive, plugin을 개발해보기로 한다.

:::note
directive는 이거다.
:::
