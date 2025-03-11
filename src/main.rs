use std::{
    path::Path,
    time::Duration,
    collections::HashMap,
    net::SocketAddr,
    convert::Infallible,
    env,
    thread,
    fs,
    sync::Arc
};
use pulldown_cmark::{
    Parser,
    Options,
    Event,
    Tag, TagEnd,
};
use yaml_rust2::{YamlLoader};
use serde_json::json;
use askama::Template;
use chrono::{ DateTime };
use toml::Table;
use axum::{http::StatusCode, Router, routing::{get,get_service}};
use tower_http::services::{ServeDir,ServeFile};
use serde_json::Value;
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo
};
use tokio::sync::{
    Mutex,
    mpsc
};

mod plugins;
use plugins::{
    MathPlugin,
    CodeHighlightPlugin,
    DirectivePlugin
};

mod utils;
use utils::{
    copy_recursively,
    is_dev
};

#[derive(Template)]
#[template(path="layout.html")]
struct Layout<'a> {
    dev: &'a bool,
    title: &'a String,
    date: &'a String,
    content: &'a String,
    description: &'a String,
}

#[derive(PartialEq, Debug)]
struct Frontmatter {
    title: String,
    date: String,
    description: String,
}

#[derive(Debug)]
struct Post {
    frontmatter: Frontmatter,
    content: String,
    slug: String,
}

const CONTENT_DIR: &str = "content";
const OUTPUT_DIR: &str = "dist";
const PUBLIC_DIR: &str = "public";

#[tokio::main]
async fn main() {
    println!("Running environment is {}.", { if is_dev() { "development" } else { "production" } });

    let config_file: String = fs::read_to_string("config.toml")
	.expect("Failed to load config file. Does file exists?");
    
    println!("{:?}", config_file.parse::<Table>().expect("Failed to parse config file."));

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

    let _ = fs::remove_dir_all(output_dir());

    let _ = copy_recursively(PUBLIC_DIR, output_dir());

    let markdown_files:Vec<String> = walkdir::WalkDir::new(CONTENT_DIR)
	.into_iter()
	.filter_map(|f| f.ok())
	.map(|f| f.path().display().to_string())
	.filter(|p| p.ends_with(".md"))
	.collect();

    println!("Building Posts: {:?}", &markdown_files);
    
    rebuild_posts(CONTENT_DIR, output_dir().as_str(), &markdown_files).expect("Build site");

    let (layer, io) = SocketIo::new_layer();
    let io = Arc::new(Mutex::new(io));

    {
        let io_clone = Arc::clone(&io);
        io_clone.lock().await.ns("/api", on_connect);
    }

    let (tx, mut rx) = mpsc::channel(100);
    
    let msg_handle_task = tokio::spawn(async move {
        let io_clone = Arc::clone(&io);

        while let Some(paths) = rx.recv().await {
            io_clone.lock().await.of("/api").unwrap().emit("refresh", &json!({"paths": paths})).await;
        }
    });


    let server_task = tokio::task::spawn(async move {
        let app = Router::new()
            // `GET /` goes to `root`
            .fallback_service(
                ServeDir::new(output_dir())
                    .not_found_service(ServeFile::new(format!("{}/404.html", output_dir()).as_str())))
            .layer(layer);

        
        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app.into_make_service()).await.unwrap()
    });

    let hotwatch_task = tokio::task::spawn(async move {
        println!("listenning for changes: {}", CONTENT_DIR);
        let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");
        hotwatch
            .watch(CONTENT_DIR, move |ev| {
                match ev.kind {
                    hotwatch::EventKind::Modify(_) => {
                        let paths: Vec<String> = ev.paths.into_iter().map(|p| {
                            p.strip_prefix(env::current_dir().unwrap())
                                .unwrap()
                                .display()
                                .to_string()
                        }).collect();

                        println!("Updated posts: {:?}", paths);

                        let web_paths: Vec<String> = paths.clone().into_iter().map(|p| {
                            p.replace(CONTENT_DIR, "").replace(".md", ".html")
                        }).collect();

                        rebuild_posts(CONTENT_DIR, output_dir().as_str(), &paths).expect("Rebuilding site");

                        tx.blocking_send(web_paths.clone()).unwrap();
                    },
                    _ => ()
                }
            })
            .expect("failed to watch content folder!");
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    tokio::try_join!(server_task, hotwatch_task, msg_handle_task).unwrap();
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    println!("Socket.IO connected. {} {}", socket.ns(), socket.id );
}


fn rebuild_posts(content_dir: &str, output_dir: &str, markdown_files: &Vec<String>) -> Result<(), anyhow::Error> {
    let mut posts: HashMap<String, Post> = HashMap::new();

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

    for file in markdown_files {
	let html_file = file
            .replace(content_dir, output_dir)
            .replace(".md", ".html");

	let post = parse_post(&file, &options, content_dir, output_dir).unwrap();

	let folder = Path::new(&html_file).parent().unwrap();
        let _ = fs::create_dir_all(folder);
        fs::write(&html_file, &post.content)?;

        posts.insert(
	    file.to_string(),
	    post
	);
    }

    Ok(())
}

fn format_date(date: &String) -> String {
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

fn parse_post(file: &str, opts: &Options, content_dir: &str, output_dir: &str) -> Result<Post, anyhow::Error> {
    let raw_markdown = fs::read_to_string(file)?;
    
    let mut raw_frontmatter = String::new();
    let mut frontmatter_started = false;

    let mut code_plugin = CodeHighlightPlugin::new();
    let math_plugin = MathPlugin::new();
    let mut directive_plugin = DirectivePlugin::new();
    
    let parser = Parser::new_ext(&raw_markdown, *opts)
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
	&raw_frontmatter, &file.to_string())
	.expect(format!("Error while exracting frontmatter from '{}'", file).as_str());

    let html_file = file
        .replace(content_dir, output_dir)
        .replace(".md", ".html");

    let html = Layout {
	dev: &is_dev(),
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

fn output_dir() -> String {
    if is_dev() {
        format!("{}/dev", OUTPUT_DIR)
    } else {
        format!("{}/production", OUTPUT_DIR)
    }
}
