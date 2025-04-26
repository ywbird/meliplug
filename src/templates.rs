use askama::Template;

#[derive(Template)]
#[template(path = "post_layout.html")]
pub struct PostLayout<'a> {
    pub dev: bool,
    pub title: &'a String,
    pub date: &'a String,
    pub content: &'a String,
    pub description: &'a String,
    pub tags: &'a Vec<String>,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeLayout<'a> {
    pub dev: bool,
    pub posts: &'a Vec<PostItem>,
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostsLayout<'a> {
    pub dev: bool,
    pub page_num: usize,
    pub max_page: usize,
    pub posts: &'a Vec<PostItem>,
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

#[derive(Debug)]
pub struct PostItem {
    pub title: String,
    pub date: String,
    pub description: Option<String>,
    pub slug: String,
}
