---
title: "자동 렌더 - Rust Blog"
date: "2025-03-05T19:59:57+09:00"
description: "파일이 변경되면 자동으로 다시 페이지를 렌더하게 해봅니다."
draft: true
---

## 필요성

파일이 변경 될때마다 프로그램을 다시 시작하고, 모든 파일을 전부 지웠다가 전부 다시 만드는건 분명 비효율적이다.  
`hotwatch` crate를 이용해 파일이 변경될 때, 변경된 파일만 다시 렌더하게 해보자.

## 만들기

여태까지는 `build_site` 함수에서 전 파일을 모두 지우고, 마크다운 파일을 파싱하고, 파일을 생성했다.  
이제는 첫 실행에만 위 동작을 진행하고, 파일 변경을 감지하여 해당 파일만 재생성 할 수 있도록, path를 받아 하나의 글 페이지만 렌더하여 객체로 반환하는 함수로 분리하였다.

```rs
fn parse_post(file: &str, opts: &Options, content_dir: &str, output_dir: &str) -> Result<Post, anyhow::Error> {
    let raw_markdown = fs::read_to_string(file)?;

    // ... parsing markdown file

    let html_file = file
        .replace(content_dir, output_dir)
        .replace(".md", ".html");

    let html = Layout {
	title: &frontmatter.title.clone(),
	date: &format_date(&frontmatter.date),
	description: &frontmatter.description.clone(),
	content: &parsed
    };

    Ok(Post {
	frontmatter,
	content: html.render().unwrap(),
	slug: html_file.replace(output_dir, ""),
    })
}
```

그리고 파싱된 markdown에서 파일을 생성하는 작업을 for문을 돌려주었다.

```rs
for file in &markdown_files {
    let html_file = file
        .replace(content_dir, output_dir)
        .replace(".md", ".html");
		
    let post = parse_post(&file, &options, content_dir, output_dir).unwrap();
	
    let folder = Path::new(&html_file).parent().unwrap();
    let _ = fs::create_dir_all(folder);
    fs::write(&html_file, &post.content)?;
}
```


