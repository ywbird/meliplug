use askama::Template;
use rusqlite::Connection;

use crate::templates::{HomeLayout, PostItem, PostsLayout};

use crate::utils::is_dev;

pub fn build_home_page() -> Result<String, anyhow::Error> {
    let conn = Connection::open("blog.db")?;

    let mut recent_posts_query = conn.prepare("SELECT * FROM posts ORDER BY date DESC LIMIT 10")?;

    let recent_posts: Vec<PostItem> = recent_posts_query
        .query_map([], |row| {
            let item = PostItem {
                title: row.get("title").unwrap(),
                date: row.get("date").unwrap(),
                description: match row.get::<_, String>("description") {
                    Ok(text) => Some(text),
                    Err(_) => None,
                },
                slug: row.get("slug").unwrap(),
            };

            Ok(item)
        })?
        .map(|post| post.unwrap())
        .collect();

    let home_page = HomeLayout {
        dev: is_dev(),
        posts: &recent_posts,
    };

    Ok(home_page.render().unwrap())
}

const POST_PER_PAGE: usize = 20;

pub fn build_posts_pages() -> Result<Vec<String>, rusqlite::Error> {
    let conn = Connection::open("blog.db")?;

    let mut post_query = conn.prepare("SELECT * FROM posts ORDER BY date DESC")?;

    let mut posts: Vec<PostItem> = post_query
        .query_map([], |row| {
            let item = PostItem {
                title: row.get("title").unwrap(),
                date: row.get("date").unwrap(),
                description: match row.get::<_, String>("description") {
                    Ok(text) => Some(text),
                    Err(_) => None,
                },
                slug: row.get("slug").unwrap(),
            };

            Ok(item)
        })?
        .map(|post| post.unwrap())
        .collect();

    posts.reverse();

    let length = posts.len();
    let page_count = length / POST_PER_PAGE;
    let mut result = Vec::new();
    for i in 1..=page_count {
        let posts: Vec<PostItem> = posts.drain(length - i * POST_PER_PAGE..).collect();
        let posts_layout = PostsLayout {
            dev: is_dev(),
            page_num: i,
            max_page: if length % POST_PER_PAGE != 0 {
                page_count + 1
            } else {
                page_count
            },
            posts: &posts,
        };
        result.push(posts_layout.render().unwrap());
    }
    if length % POST_PER_PAGE != 0 {
        let posts_layout = PostsLayout {
            dev: is_dev(),
            page_num: page_count + 1,
            max_page: page_count + 1,
            posts: &posts,
        };
        result.push(posts_layout.render().unwrap());
    }

    Ok(result)
}
