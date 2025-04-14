use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::HighlightEvent;
use tree_sitter_highlight::Highlighter;
use tree_sitter_highlight::Highlight;
use std::fmt::Write;

#[derive(PartialEq, Eq, Hash)]
pub enum Lang {
    Rust
}

pub fn highlight_code (src: &[u8], conf: &HighlightConfiguration) -> String {
    let mut highlighter = Highlighter::new();
    
    let highlights = highlighter.highlight(
        conf,
        src,
        None,
        |_| None
    ).unwrap();

    let mut result = String::new();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source {start, end} => {
                write!(result, "{}", String::from_utf8(src[start..end].to_vec()).unwrap()).unwrap();
            },
            HighlightEvent::HighlightStart(Highlight(i)) => {
                write!(result, r#"<span class=hc{}>"#, i).unwrap();
            },
            HighlightEvent::HighlightEnd => {
                write!(result, r#"</span>"#).unwrap();
            },
        }
    }

    result
}
