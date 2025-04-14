use std::convert::From;
use std::fs;
use std::io::{Error as IOError, Write};
use std::string::FromUtf8Error;
use std::collections::HashMap;
use orgize::{Element, Org};
use orgize::elements::{SourceBlock, Keyword};
use orgize::export::{DefaultHtmlHandler, HtmlHandler};
use slugify::slugify;
use tree_sitter_highlight::HighlightConfiguration;
use askama::Template;

use crate::templates::{Frontmatter, Post, PostLayout};

use crate::highlight::{highlight_code, Lang};

use crate::utils::{format_date, is_dev};

#[derive(Debug)]
enum OrgError {
    IO(IOError),
    Heading,
    Utf8(FromUtf8Error),
}

impl From<IOError> for OrgError {
    fn from(err: IOError) -> Self {
        OrgError::IO(err)
    }
}

#[derive(Default)]
struct CustomHtmlHandler {
    default: DefaultHtmlHandler,
    current_level: usize,
    highlight_langs: HashMap<Lang, HighlightConfiguration>,
    tags: Vec<String>
}

impl CustomHtmlHandler {
    fn new(langs: HashMap<Lang, HighlightConfiguration>) -> Self {

        Self {
            default: Default::default(),
            current_level: 0,
            highlight_langs: langs,
            tags: Vec::new(),
        }
    }
}

impl HtmlHandler<OrgError> for CustomHtmlHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), OrgError> {
        println!("START: {:#?}", &element);
        
        match element {
            Element::Title(title) => {
                if title.raw == "TAGS" {
                    self.tags = title.tags.iter().map(|t| t.to_string()).collect();
                } else {
                    if title.level > 6 {
                        return Err(OrgError::Heading);
                    }
                    self.current_level = title.level;

                    write!(
                        w,
                        "<h{0} data-level=\"{1}\"><a id=\"{0}.{2}\" href=\"#{0}.{2}\">",
                        title.level + 1, title.level, slugify!(&title.raw)
                    )?;
                }
            },
            Element::SourceBlock(SourceBlock{
                contents,
                language,
                arguments,
                ..
            }) => {
                write!(w, "<pre><code data-language=\"{}\">", language)?;
                write!(w, "{}", match language.to_lowercase().as_str() {
                    "rust" => {
                        highlight_code(
                            contents.split("\n")
                            .map(|l| l.strip_prefix("  ").unwrap_or(l))
                            .collect::<Vec<&str>>().join("\n").as_bytes(),
                            &self.highlight_langs.get(&Lang::Rust).unwrap()
                        )
                    },
                    _ => { contents.to_string() }
                })?;
            },
            Element::Section => {
                write!(w, "<section data-level=\"{}\">", self.current_level)?;
            },
            _ => self.default.start(w, element)?
        }

        Ok(())
    }
    
    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), OrgError> {
        println!("END: {:#?}", element);

        match element {
            Element::Title(title) => {
                write!(w, "</a></h{}>", title.level + 1)?;
            },
            Element::SourceBlock(_) => {
                write!(w, "</pre></code>")?;
            },
            _ => self.default.end(w, element)?
        }
        Ok(())
    }
}

pub fn parse_org_post(
    file: &str,
    content_dir: &str,
    output_dir: &str,
) -> Result<Post, anyhow::Error> {
    let raw_org = fs::read_to_string(file)?;

    let langs = configure_tree_sitter();
    
    let mut writer = Vec::new();
    let mut handler = CustomHtmlHandler::new(langs);
    
    let parser = Org::parse(raw_org.as_str());

    parser.write_html_custom(&mut writer, &mut handler).unwrap();

    let mut properties: HashMap<String, String> = HashMap::new();
    for keyword in parser.keywords() {
        properties.insert(keyword.key.to_string(), keyword.value.to_string());
    }
    
    let parsed = String::from_utf8(writer[6..writer.len()-7].to_vec()).unwrap();

    println!("{}", parsed);

    let Some(title) = properties.get("TITLE") else {
        panic!("Error while processing {}. Post must have an `TITLE` property.", &file);
    };

    let Some(date) = properties.get("DATE") else {
        panic!("Error while processing {}. Post must have an `DATE` property.", &file);
    };

    let description = match properties.get("DESCRIPTION") {
        Some(text) => text,
        None => ""
    };

    let html_file = file
        .replace(content_dir, output_dir)
        .replace(".org", ".org.html");

    let frontmatter = Frontmatter {
        title: title.to_string(),
        date: date.to_string(),
        description: description.to_string(),
        tags: handler.tags,
    };

    let html = PostLayout {
        dev: &is_dev(),
        title: &frontmatter.title.clone(),
        date: &format_date(&frontmatter.date),
        description: &frontmatter.description.clone(),
        content: &parsed,
        tags: &frontmatter.tags.clone(),
    };
    
    Ok(Post {
        frontmatter,
        content: html.render().unwrap(),
        slug: html_file,
        raw: raw_org
    })
}

fn configure_tree_sitter() -> HashMap<Lang, HighlightConfiguration> {
        let mut langs: HashMap<Lang, HighlightConfiguration> = HashMap::new();

    langs.insert(Lang::Rust, {
        let rust_language = tree_sitter_rust::LANGUAGE;

        let highlight_names = [
            "attribute",
            "comment",
            "constant",
            "constant.builtin",
            "constructor",
            "embedded",
            "function",
            "function.builtin",
            "keyword",
            "module",
            "number",
            "operator",
            "property",
            "property.builtin",
            "punctuation",
            "punctuation.bracket",
            "punctuation.delimiter",
            "punctuation.special",
            "string",
            "string.special",
            "tag",
            "type",
            "type.builtin",
            "variable",
            "variable.builtin",
            "variable.parameter",
        ];
            
        let mut rust_config = HighlightConfiguration::new(
            rust_language.into(),
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            "",
        ).unwrap();

        rust_config.configure(&highlight_names);
            
        rust_config
    });

    langs
}
