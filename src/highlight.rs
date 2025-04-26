use std::fmt::Write;
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::HighlightEvent;
use tree_sitter_highlight::Highlighter;
use tree_sitter_highlight::Highlight;

#[derive(PartialEq, Eq, Hash)]
pub enum Lang {
    Rust,
    JavaScript,
    Html,
    Ansi,
    Lua,
    Json,
    Bash
}

pub fn find_lang(lang: &str) -> Option<Lang> {
    match lang.trim() {
        "rust"|"rs" => Some(Lang::Rust),
        "js"|"javascript" => Some(Lang::JavaScript),
        "lua" => Some(Lang::JavaScript),
        "json" => Some(Lang::JavaScript),
        "bash" => Some(Lang::Bash),
        "html" => Some(Lang::Html),
        "ansi" => Some(Lang::Ansi),
        _ => None
    }
}

pub fn highlight_code (src: &[u8], lang: Option<Lang>) -> String {
    let mut highlighter = Highlighter::new();

    match lang {
        Some(Lang::Ansi) => {
            ansi_to_html::convert(&String::from_utf8(src.to_vec()).unwrap()).unwrap()
        },
        Some(lang) => {
            let conf = configure_tree_sitter(lang);
    
            let highlights = highlighter.highlight(
                &conf,
                src,
                None,
                |_| None
            ).unwrap();

            let mut result = String::new();

            write!(result, r#"<div class="h_line">"#).unwrap();
    
            for event in highlights {
                match event.unwrap() {
                    HighlightEvent::Source {start, end} => {
                        write!(result, "{}",
                            String::from_utf8(src[start..end].to_vec())
                            .unwrap()
                            .replace("<","&lt;")
                            .replace(">","&gt;")
                            .replace("\n", r#"</div><div class="h_line">"#)
                        ).unwrap();
                    },
                    HighlightEvent::HighlightStart(Highlight(i)) => {
                        write!(result, r#"<span class="h_{:02}">"#, i).unwrap();
                    },
                    HighlightEvent::HighlightEnd => {
                        write!(result, r#"</span>"#).unwrap();
                    }
                }
            }

            write!(result, r#"</div>"#).unwrap();

            result
        },
        None => {
            String::from_utf8(src.to_vec()).unwrap()
                .replace("<","&lt;")
                .replace(">","&gt;")
        }
    }

}


fn configure_tree_sitter(lang: Lang) -> HighlightConfiguration {
    let highlight_names = [
        "attribute",
        "comment",
        "constant",
        "constant.builtin",
        "constructor",
        "embedded",
        "function",
        "function.builtin",
        "function.macro",
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
    
    match lang {
        Lang::Rust => {
            let rust_language = tree_sitter_rust::LANGUAGE;
            
            let mut rust_config = HighlightConfiguration::new(
                rust_language.into(),
                "rust",
                tree_sitter_rust::HIGHLIGHTS_QUERY,
                tree_sitter_rust::INJECTIONS_QUERY,
                "",
            ).unwrap();

            rust_config.configure(&highlight_names);
            
            rust_config
        },
        Lang::JavaScript => {
            let js_language = tree_sitter_javascript::LANGUAGE;
            
            let mut js_config = HighlightConfiguration::new(
                js_language.into(),
                "javascript",
                tree_sitter_javascript::HIGHLIGHT_QUERY,
                tree_sitter_javascript::INJECTIONS_QUERY,
                tree_sitter_javascript::LOCALS_QUERY,
            ).unwrap();

            js_config.configure(&highlight_names);
            
            js_config
        },
        Lang::Lua => {
            let lua_language = tree_sitter_lua::LANGUAGE;
            
            let mut lua_config = HighlightConfiguration::new(
                lua_language.into(),
                "lua",
                tree_sitter_lua::HIGHLIGHTS_QUERY,
                tree_sitter_lua::INJECTIONS_QUERY,
                tree_sitter_lua::LOCALS_QUERY,
            ).unwrap();

            lua_config.configure(&highlight_names);
            
            lua_config
        },
        Lang::Json => {
            let json_language = tree_sitter_json::LANGUAGE;
            
            let mut json_config = HighlightConfiguration::new(
                json_language.into(),
                "json",
                tree_sitter_json::HIGHLIGHTS_QUERY,
                "",
                ""
            ).unwrap();

            json_config.configure(&highlight_names);
            
            json_config
        },
        Lang::Bash|Lang::Ansi => {
            let bash_language = tree_sitter_bash::LANGUAGE;
            
            let mut bash_config = HighlightConfiguration::new(
                bash_language.into(),
                "bash",
                tree_sitter_bash::HIGHLIGHT_QUERY,
                "",
                "",
            ).unwrap();

            bash_config.configure(&highlight_names);
            
            bash_config
        },
        Lang::Html => {
            let html_language = tree_sitter_html::LANGUAGE;
            
            let mut html_config = HighlightConfiguration::new(
                html_language.into(),
                "html",
                tree_sitter_html::HIGHLIGHTS_QUERY,
                tree_sitter_html::INJECTIONS_QUERY,
                "",
            ).unwrap();

            html_config.configure(&highlight_names);
            
            html_config
        },
    }
}
