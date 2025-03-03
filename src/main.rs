use std::{fs, path::Path};
use pulldown_cmark::{
    Parser,
    Options,
    Event,
    Tag, TagEnd,
    TextMergeStream
};
use yaml_rust2::{YamlLoader};
use askama::Template;
use chrono::{ DateTime };
mod plugins;
use plugins::{
    MathPlugin,
    CodeHighlightPlugin,
    DirectivePlugin
};

#[derive(Template)]
#[template(path="layout.html")]
struct Layout<'a> {
    title: &'a String,
    content: &'a String,
}

#[derive(PartialEq, Debug)]
struct Frontmatter {
    title: String,
    date: String,
    description: String,
}

const CONTENT_DIR: &str = "content";
const OUTPUT_DIR: &str = "dist";
const PUBLIC_DIR: &str = "public";

#[tokio::main]
async fn main() {
    build_post(CONTENT_DIR, OUTPUT_DIR, PUBLIC_DIR).expect("Build site");
}

fn build_post(content_dir: &str, output_dir: &str, public_dir: &str) -> Result<(), anyhow::Error> {
    let _ = fs::remove_dir_all(output_dir);

    let _ = copy_recursively(public_dir, output_dir);

    let markdown_files:Vec<String> = walkdir::WalkDir::new(content_dir)
	.into_iter()
	.filter_map(|f| f.ok())
	.map(|f| f.path().display().to_string())
	.filter(|p| p.ends_with(".md"))
	.collect();

    let mut html_files: Vec<String> = Vec::with_capacity(markdown_files.len());

    for file in &markdown_files {
	let raw_markdown = fs::read_to_string(&file)?;

	let options = {
	    let mut opt = Options::empty();

	    opt.insert(Options::ENABLE_STRIKETHROUGH);
	    opt.insert(Options::ENABLE_TABLES);
	    opt.insert(Options::ENABLE_TASKLISTS);
	    opt.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
	    opt.insert(Options::ENABLE_GFM);
	    opt.insert(Options::ENABLE_HEADING_ATTRIBUTES);
	    opt.insert(Options::ENABLE_MATH);

	    opt
	};
	
	let mut raw_frontmatter = String::new();
	let mut frontmatter_started = false;

	let mut code_plugin = CodeHighlightPlugin::new();
	let math_plugin = MathPlugin::new();
	let mut directive_plugin = DirectivePlugin::new();
	
	let parser = Parser::new_ext(&raw_markdown, options)
	    .map(|event| { // frontmatter 
		match event {
		    Event::Start(Tag::MetadataBlock(text)) => {
			frontmatter_started = true;
			Event::Start(Tag::MetadataBlock(text))
		    },
		    Event::End(TagEnd::MetadataBlock(text)) => {
			frontmatter_started = false;
			Event::End(TagEnd::MetadataBlock(text))
		    },
		    Event::Text(text) => {
			if frontmatter_started {
			    let _ = &raw_frontmatter.push_str(&text);
			}
			Event::Text(text)
		    },
		    _ => event
		}
	    })
	    .map(math_plugin.apply())
	    .map(code_plugin.apply())
	    .map(directive_plugin.apply());

	let mut parsed  = String::new();

	pulldown_cmark::html::push_html(&mut parsed, parser);
	
	let frontmatter = extract_frontmatter(
		&raw_frontmatter, &file)
		.expect(format!("Error while exracting frontmatter from '{}'", &file).as_str());

	let html_file = file
            .replace(content_dir, output_dir)
            .replace(".md", ".html");

	let html = Layout { title: &frontmatter.title, content: &parsed };

	let folder = Path::new(&html_file).parent().unwrap();
        let _ = fs::create_dir_all(folder);
        fs::write(&html_file, html.render().unwrap())?;

        html_files.push(html_file);
    }

    println!("{:?}", markdown_files);

    Ok(())
}

fn _format_date(date: &String) -> String {
    format!("{}", DateTime::parse_from_rfc3339(date.as_str()).expect("Error while parsing date").format("%d %b %Y"))
}



fn extract_frontmatter(yaml: &String, file: &String) -> Result<Frontmatter, ()> {
    let values = &(YamlLoader::load_from_str(yaml).unwrap())[0];
    let title = values["title"]
	.as_str()
	.expect(format!("Unable to read 'title' from frontmatter of '{}'.", &file).as_str())
	.to_string();
    let date = values["date"]
	.as_str()
	.expect(format!("Unable to read 'date' from frontmatter of '{}'.", &file).as_str())
	.to_string();
    let description = values["description"]
	.as_str()
	.expect(format!("Unable to read 'description' from frontmatter of '{}'.", &file).as_str())
	.to_string();

    Ok(Frontmatter {
	title,
	date,
	description
    })
}

/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

