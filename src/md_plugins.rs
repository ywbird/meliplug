use regex::Regex;
use katex::opts::OutputType;
use syntect::{
    parsing::SyntaxSet,
    highlighting::ThemeSet,
    html::highlighted_html_for_string
};
use pulldown_cmark::{
    Event,
    Tag, TagEnd,
    CodeBlockKind
};
use std::collections::HashMap;
use std::fmt::Write;

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
			.output_type(OutputType::Html)
			.build().unwrap();
		    let html = katex::render_with_opts(&text, &opts).unwrap();
		    Event::Html(html.into())
		},
		Event::DisplayMath(text) => {
		    let opts = katex::Opts::builder()
			.display_mode(true)
			.trust(true)
			.output_type(OutputType::Html)
			.build().unwrap();
		    let html = katex::render_with_opts(&text, &opts).unwrap();
		    Event::Html(html.into())
		}, 
		_ => event
	    }
	}
    }
}

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
    pub fn apply(&mut self) -> impl FnMut(Event<'_>) -> Event<'_> + use<'_> {
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
		    let ps = SyntaxSet::load_defaults_newlines();
		    let ts = ThemeSet::load_defaults();

		    let syntax = match ps.find_syntax_by_extension(self.lang.as_str()) {
			Some(s) => s,
			None => ps.find_syntax_plain_text(),
		    };
		    let theme = &ts.themes["base16-ocean.dark"];
		    let html = highlighted_html_for_string(self.source.as_str(), &ps, syntax, theme);
		    
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

#[derive(Debug, Clone)]
struct Directive {
    name: String,
    tag_name: String,
    raw: String,
    // attributes: String,
    attributes: HashMap<String, String>,
}

#[allow(unused_must_use)]
impl Directive {
    pub fn open(&self) -> String {
	let mut tag = "<".to_string();
	write!(&mut tag, "{} ", self.tag_name);
	write!(&mut tag, "data-name=\"{}\" ", self.name);
	for (key, val) in self.attributes.iter() {
	    write!(&mut tag, "{}=\"{}\" ", key, val);
	}
	write!(&mut tag, ">");
	tag
    }

    pub fn close(&self) -> String {
	let mut tag = "</".to_string();
	write!(&mut tag, "{}", self.tag_name);
	write!(&mut tag, ">");
	tag 
    }
}

pub struct DirectivePlugin {
    stack: Vec<Directive>,
}

impl DirectivePlugin {
    pub fn new() -> Self {
	Self {
	    stack: Vec::new()
	}
    }

    pub fn apply(&mut self) -> impl FnMut(Event<'_>) -> Event<'_> + use<'_> {
	return |event| {
	    match &event {
		Event::Text(text) => {
		    let re = Regex::new(r#":::(?<name>\w+)?\s?(?<attr>\{(.*)\})?"#).unwrap();
		    if re.is_match(&text) {
                        let prefix = ":::".to_string();
		        if prefix == text.to_string() {
			    if self.stack.len() >= 1 {
			        return Event::Html(self.stack.pop().unwrap().close().into());
			    }
		        } else {
			    let new_directive = resolve_directive(&text.to_string());
			    self.stack.push(new_directive.clone());
			    return Event::Html(new_directive.open().into());
		        }
                    }
		},
		_ => (),
	    };

	    event
	}
    }

    pub fn _apply_text_directive() -> impl FnMut(Event<'_>) -> Event<'_> {
        return |event| {
	    match &event {
	        Event::Text(text) => {
                    let re = Regex::new(r#"(?<dir>:[a-z]+(\{.*\})?)"#).unwrap();
		    for directive in re.captures_iter(&text).map(|d| d.name("dir").unwrap().as_str()) {
                        println!("{}", directive);
		    }
	        },
	        _ => ()
	    }

            event
        }
    }
}

fn resolve_directive(start: &String) -> Directive {
    let re = Regex::new(r#":{1,3}(?<name>\w+)?\s?(\{(?<attr>.*)\})?"#).unwrap();
    let result = re.captures(start).unwrap();
    let name = match result.name("name") {
	Some(s) => s.as_str().to_string(),
	None => "div".to_string()
    };
    let raw_attrs = match result.name("attr") {
	Some(s) => s.as_str(),
	None => ""
    };
    let mut tag_name = "div".to_string();
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert("class".to_string(), "".to_string());
    for term in raw_attrs.split_whitespace() {
	let re_class_id = Regex::new(r#"(\.[a-z0-9\-]+)+"#).unwrap();
	let re_val = Regex::new(r#"(?<name>[a-z\-]*)\=\"(?<val>.*)\""#).unwrap();

	if re_class_id.is_match(&term) {
	    let re_item = Regex::new(r#"\.(?<name>[a-z0-9\-]+)"#).unwrap();
	    for (_, [name]) in re_item.captures_iter(&term).map(|c| c.extract()) {
		if attributes.get("class").unwrap().len() != 0 {
		    attributes.get_mut("class").unwrap().push_str(" ");
		}
		attributes.get_mut("class").unwrap().push_str(name);
	    }
	} else if re_val.is_match(&term) {
	    let (_, [key, value]) = re_val.captures(&term).unwrap().extract();
	    if key == "tag" {
		tag_name = value.to_string();
	    } else {	
		attributes.insert(key.to_string(), value.to_string());
	    }
	}
    }
    return Directive {
	name,
	tag_name,
	attributes,
        raw: Default::default()
    }
}
