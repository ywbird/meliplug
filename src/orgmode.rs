use askama::Template;
use orgize::export::{from_fn_with_ctx, Container, Event, HtmlExport, Traverser};
use orgize::Org;
use slugify::slugify;
use std::collections::HashMap;
use std::fs;

use crate::templates::{Frontmatter, Post, PostLayout};

use crate::highlight::{find_lang, highlight_code};

use crate::utils::{format_date, is_dev};

pub fn parse_org_post(
    file: &str,
    content_dir: &str,
    output_dir: &str,
) -> Result<Post, anyhow::Error> {
    let raw_org = fs::read_to_string(file)?;

    let mut html_export = HtmlExport::default();

    let mut current_level: usize = 0;
    let mut in_folded = false;
    let mut in_source = false;
    let mut last_linenum = 0;

    let mut handler = from_fn_with_ctx(|event, ctx| {
        // println!("{:?}", &event);

        match event {
            Event::Enter(Container::Headline(title)) => {
                if title.level() > 6 {
                    current_level = 5;
                } else {
                    current_level = title.level();
                }

                if title
                    .tags()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .contains(&"FOLDED".to_string())
                {
                    in_folded = true;
                }

                html_export.push_str(format!(
                    "<h{0} data-level=\"{1}\"><a id=\"{0}.{2}\" href=\"#{0}.{2}\">",
                    current_level + 1,
                    current_level,
                    slugify!(&title.title_raw())
                ));
                for elem in title.title() {
                    html_export.element(elem, ctx);
                }
                html_export.push_str(format!("</a></h{}>", current_level + 1));
            }
            Event::Enter(Container::SourceBlock(source)) => {
                html_export.push_str(format!(
                    "<pre class=\"codeblock\"><code data-language=\"{}\" >",
                    source.language().unwrap()
                ));

                html_export.push_str(format!(
                    "{}",
                    highlight_code(
                        source
                            .value()
                            .split("\n")
                            .map(|l| {
                                last_linenum += 1;
                                l.strip_prefix("  ").unwrap_or(l)
                            })
                            .collect::<Vec<&str>>()
                            .join("\n")
                            .as_bytes(),
                        find_lang(source.language().unwrap().to_lowercase().as_str()),
                    )
                ));

                // println!("params: {}", match source.parameters() { Some(s) => s.to_string(), None => "".to_string() });

                in_source = true;
            }
            Event::Text(_) => {
                if !in_source {
                    html_export.event(event, ctx)
                }
            }
            Event::Leave(Container::SourceBlock(_)) => {
                html_export.push_str(format!("</pre></code>"));
                in_source = false;
            }
            Event::Enter(Container::Section(_)) => {
                html_export.push_str(format!(
                    "<{} class=\"section\" data-level=\"{}\">",
                    if in_folded { "details" } else { "section" },
                    current_level
                ));
            }
            Event::Leave(Container::Section(_)) => {
                html_export.push_str(format!(
                    "</{}>",
                    if in_folded { "details" } else { "section" }
                ));
                in_folded = false;
            }
            _ => html_export.event(event, ctx),
        }
    });

    let parser = Org::parse(&raw_org);

    parser.traverse(&mut handler);

    let mut properties: HashMap<String, String> = HashMap::new();
    for keyword in parser.keywords() {
        properties.insert(
            keyword.key().to_string(),
            keyword.value().trim().to_string(),
        );
    }

    let parsed = html_export.finish();
    let parsed = parsed[6..parsed.len() - 7].to_string();

    let Some(title) = parser.title() else {
        panic!(
            "Error while processing {}. Post must have an `TITLE` property.",
            &file
        );
    };

    let date = match properties.get("DATE") {
        Some(text) => text,
        None => "",
    };

    let description = match properties.get("DESCRIPTION") {
        Some(text) => text,
        None => "",
    };

    let html_file = file
        .replace(content_dir, output_dir)
        .replace(".org", ".org.html");

    let frontmatter = Frontmatter {
        title: title.to_string(),
        date: date.to_string(),
        description: description.to_string(),
        tags: Vec::new(),
    };

    let html = PostLayout {
        dev: is_dev(),
        title: &frontmatter.title.clone(),
        date: if date.len() > 0 {
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
        raw: raw_org,
    })
}
