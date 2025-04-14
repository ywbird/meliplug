use askama::Template;

#[derive(Template)]
#[template(path = "post_layout.html")]
pub struct PostLayout<'a> {
    pub dev: &'a bool,
    pub title: &'a String,
    pub date: &'a String,
    pub content: &'a String,
    pub description: &'a String,
    pub tags: &'a Vec<String>,
}

#[derive(PartialEq, Debug)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    pub description: String,
    pub tags: Vec<String>,
}
 
#[derive(Debug)]
pub struct Post {
    pub frontmatter: Frontmatter,
    pub content: String, 
    pub slug: String,
    pub raw: String,
}
