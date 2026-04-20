use std::fs;
use std::io::Error;
use std::path::Path;
use std::collections::HashMap;

use tera::Tera;
use tera::Context;

use serde::Serialize;

const TAG_H1: &str = "# ";

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
}

pub fn load(path: &Path) -> Result<Post<'_>, Error> {
    let mut post = Post::new(path);
    parse(&mut post);
    post.html = markdown::to_html(&post.text);

    Ok(post)
}

// fn read_attrs(path: &Path) -> HashMap<String, String> {
fn parse(post: &mut Post) {
    let new_line: String = '\n'.to_string();
    let mut found = false;
    let mut text = String::from("");

    for line in fs::read_to_string(&post.path).unwrap().lines() {
        if !found && line.starts_with(TAG_H1) {
            found = true;
        }

        if found {
            text = text + &new_line + line;
            continue;
        }

        let line = line.to_owned();
        if !line.starts_with("<!--") {
            continue;
        }

        let bare = line.replace("<!--", "").replace("-->", "");
        let mut field = bare.clone().trim().to_string();
        let mut value = "true".to_string();
        if let Some(colon) = bare.find(":") {
            field = bare[0..colon].to_string().trim().to_string();
            value = bare[colon+1..].to_string().trim().to_string();
        }
        post.attrs.insert(field.clone(), value.to_string());
    }

    post.text = text;
}

pub fn render(post: &Post, file: impl AsRef<Path>) {
    // Yes, this is very stupid but I don't feel lazy enough today.
    let tera = match Tera::new("src/templates/**/*.html") {
        Ok(t) => t,
        Err(e) => panic!("Error parsing templates: {}.\n", e),
    };

    let mut context = Context::new();
    context.insert("post", &post);
    let text = match tera.render("views/post.html", &context) {
        Ok(s) => s,
        Err(e) => panic!("Failed to render: {}", e),
    };

    fs::write(file, &text).expect("Failed to write")
}
