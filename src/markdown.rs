use askama::Template;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::fs;
use yaml_rust2::YamlLoader;

use crate::md_plugins::{CodeHighlightPlugin, DirectivePlugin, MathPlugin};

use crate::templates::{Frontmatter, Post, PostLayout};

use crate::utils::{format_date, is_dev};

pub fn parse_md_post(
    file: &str,
    opts: &Options,
    content_dir: &str,
    output_dir: &str,
) -> Result<Post, anyhow::Error> {
    let raw_markdown = fs::read_to_string(file)?;

    let mut raw_frontmatter = String::new();
    let mut frontmatter_started = false;

    let mut code_plugin = CodeHighlightPlugin::new();
    let math_plugin = MathPlugin::new();
    let mut directive_plugin = DirectivePlugin::new();

    let parser = Parser::new_ext(&raw_markdown, *opts)
        .map(|event| {
            // frontmatter
            match event {
                Event::Start(Tag::MetadataBlock(text)) => {
                    frontmatter_started = true;
                    Event::Start(Tag::MetadataBlock(text))
                }
                Event::End(TagEnd::MetadataBlock(text)) => {
                    frontmatter_started = false;
                    Event::End(TagEnd::MetadataBlock(text))
                }
                Event::Text(text) => {
                    if frontmatter_started {
                        let _ = &raw_frontmatter.push_str(&text);
                    }
                    Event::Text(text)
                }
                _ => event,
            }
        })
        .map(math_plugin.apply())
        .map(code_plugin.apply())
        .map(directive_plugin.apply());

    let mut parsed = String::new();

    pulldown_cmark::html::push_html(&mut parsed, parser);

    let frontmatter = extract_frontmatter(&raw_frontmatter, &file.to_string())
        .expect(format!("Error while exracting frontmatter from '{}'", file).as_str());

    let html_file = file
        .replace(content_dir, output_dir)
        .replace(".md", ".md.html");

    let html = PostLayout {
        dev: is_dev(),
        title: &frontmatter.title.clone(),
        date: if frontmatter.date.len() > 0 {
            &format_date(&frontmatter.date)
        } else {
            &"".to_string()
        },
        description: &frontmatter.description.clone(),
        content: &parsed,
        tags: &frontmatter.tags.clone(),
    };

    Ok(Post {
        frontmatter,
        content: html.render().unwrap(),
        slug: html_file.replace(output_dir, ""),
        raw: raw_markdown.clone(),
    })
}

fn extract_frontmatter(yaml: &String, file: &String) -> Result<Frontmatter, ()> {
    let values = &(YamlLoader::load_from_str(yaml).unwrap())[0];
    let title = values["title"]
        .as_str()
        .expect(format!("Unable to read 'title' from frontmatter of '{}'.", &file).as_str())
        .to_string();

    let date = match values["date"].as_str() {
        Some(text) => text,
        None => values["publish_date"].as_str().unwrap_or_default(),
    }
    .to_string();

    let description = match values["description"].as_str() {
        Some(text) => text,
        None => "",
    }
    .to_string();

    let tags: Vec<String> = match values["tags"].clone().into_vec() {
        Some(t) => t
            .into_iter()
            .map(|p| p.as_str().unwrap().to_string())
            .collect(),
        None => Vec::new(),
    };

    Ok(Frontmatter {
        title,
        date,
        description,
        tags,
    })
}
