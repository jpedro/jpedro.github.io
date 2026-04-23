use std::fs;
use std::io::Error;
use std::path::Path;
use std::collections::HashMap;

use serde::Serialize;
use tera::Context;

use crate::render::Renderer;

const TAG_H1: &str = "# ";
const COMMENT_OPEN: &str = "<!--";
const COMMENT_CLOSE: &str = "-->";
const SLASH: &str = "/";

#[derive(Serialize)]
pub struct Post<'a> {
    pub path: &'a str,
    pub text: String,
    pub html: String,
    pub title: &'a str,
    pub attrs: HashMap<String, String>,
}

impl Post<'_> {
    pub fn new(path: &Path) -> Post<'_> {
        Post {
            path: &path.to_str().unwrap(),
            text: "".into(),
            html: "".into(),
            title: &path.file_name().unwrap().to_str().unwrap(),
            attrs: HashMap::new(),
        }
    }

    /// Gets the final html file the post should be rendered into.
    pub fn dest(&self, dir: &str) -> String {
        format!(
            "docs/{}.html",
            self.path.
                replace(".md", "").
                replace(&dir, "").
                strip_prefix(SLASH)
                .unwrap()
        )
    }

    pub fn render(&self, renderer: &Renderer, file: impl AsRef<Path>) {
        let mut context = Context::new();
        context.insert("post", &self);
        let text = renderer.render("views/post.html", &context);
        fs::write(file, &text).expect("Failed to write")
    }

}

/// Yeah, this should be also moved inside impl
pub fn load(path: &Path) -> Result<Post<'_>, Error> {
    let mut post = Post::new(path);
    parse(&mut post);
    post.html = markdown::to_html(&post.text);

    Ok(post)
}

/// Yeah, this should be also moved inside impl
fn parse(post: &mut Post) {
    let new_line: String = "\n".to_string();
    let mut found_h1 = false;
    let mut text = String::from("");

    for line in fs::read_to_string(&post.path).unwrap().lines() {
        if !found_h1 && line.starts_with(TAG_H1) {
            found_h1 = true;
        }

        if found_h1 {
            text = text + &new_line + line;
            continue;
        }

        let line = line.to_owned();
        if !line.starts_with(COMMENT_OPEN) {
            continue;
        }

        let bare = line.replace(COMMENT_OPEN, "").replace(COMMENT_CLOSE, "");
        let mut field = bare.clone().trim().to_string();
        let mut value = "true".to_string();
        if let Some(colon) = bare.find(":") {
            field = bare[0..colon].to_string().trim().into();
            value = bare[colon+1..].to_string().trim().into();
        }
        post.attrs.insert(field.clone(), value.to_string());
    }

    post.text = text;
}
