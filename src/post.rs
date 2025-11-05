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

pub fn load(path: &Path) -> Result<Post<'_>, Error> {
    // for line in post.text.split("\n") {
    //     if found {
    //         lines.push(line);
    //     } else if line.starts_with(TAG_H1) {
    //         println!("> # Found the H1 tag: '{}'", line);
    //         post.title = &line.replace("# from", "");
    //         found = true;
    //     } else if line.starts_with("<!--") {
    //         let front = line.replace("<!--", "").replace("-->", "");
    //         println!("> Found front: '{}'", front);
    //         let mut field = line;
    //         let mut value = "true";
    //         if let Some(colon) = line.find(":") {
    //             println!("> Found colon: '{}'", colon);
    //             field = &line[0..colon];
    //             value = &line[colon+1..];
    //         }
    //         attrs.insert(field, value);
    //         println!("> Front field: '{}', value: '{}'", field, value);
    //     } else {
    //         println!("> What is this: '{}'", line);
    //     }
    // }

    let (text, attrs) = parse(&path);
    let html = markdown::to_html(&text);

    let post = Post {
        path: &path.to_str().unwrap(),
        text: text,
        html: html,
        title: &path.file_name().unwrap().to_str().unwrap(),
        attrs: attrs,
        // lines: read_lines(&path),
    };

    Ok(post)
}

// fn read_text(path: &Path) -> String {
//     fs::read_to_string(path).unwrap()
// }

// fn read_lines(path: &Path) -> Vec<String> {
//     fs::read_to_string(path)
//         .unwrap()
//         .lines()
//         .map(|line: &str| line.into())
//         .collect()
// }

// fn read_attrs(path: &Path) -> HashMap<String, String> {
fn parse(path: &Path) -> (String, HashMap<String, String>) {
    let new_line: String = '\n'.to_string();
    let mut found = false;
    let mut text: String = String::from("");
    let mut attrs: HashMap::<String, String> = HashMap::new();

    for line in fs::read_to_string(path).unwrap().lines() {
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
        // println!("> Using bare: '{}'.", bare);
        let mut field = bare.clone().trim().to_string();
        let mut value = "true".to_string();
        if let Some(colon) = bare.find(":") {
            // println!("> Found colon: {} on '{}'", colon, bare);
            field = bare[0..colon].to_string().trim().to_string();
            value = bare[colon+1..].to_string().trim().to_string();
        }
        attrs.insert(field.clone(), value.to_string());
        // println!("  {}: {}", field, value);
    }

    (text, attrs)
}

pub fn render(post: &Post, file: impl AsRef<Path>) {
    let tera = match Tera::new("templates/**/*.html") {
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
