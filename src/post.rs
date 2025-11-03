use std::fs;
use std::io::Error;
use std::path::Path;

use std::collections::HashMap;

const TAG_H1: &str = "# ";

// #[allow(dead_code)]
pub struct Post<'a> {
    pub path: &'a Path,
    pub text: String,
    pub lines: Vec<&'a str>,
    pub title: &'a str,
    pub attrs: HashMap<&'a str, &'a str>,
}

pub fn process(path: &Path) -> Result<Post<'_>, Error> {
    let text = fs::read_to_string(path).expect("Couldn't read file");

    let mut post = Post {
        path: &path,
        text: text,
        title: &path
            .file_name()
            .expect("Failed to get file name")
            .to_string_lossy(),
        lines: vec![],
        attrs: HashMap::new(),
    };
    load(&mut post);

    Ok(post)
}

pub fn load<'a>(post: &'a mut Post<'a>) {
    let mut attrs: HashMap::<&str, &str> = HashMap::new();
    let mut lines: Vec<&str> = Vec::new();
    let mut found = false;

    for line in post.text.split("\n") {
        if found {
            lines.push(line);
        } else if line.starts_with(TAG_H1) {
            println!("> # Found the H1 tag: '{}'", line);
            post.title = &line.replace("# from", "");
            found = true;
        } else if line.starts_with("<!--") {
            let front = line.replace("<!--", "").replace("-->", "");
            println!("> Found front: '{}'", front);
            let mut field = line;
            let mut value = "true";
            if let Some(colon) = line.find(":") {
                println!("> Found colon: '{}'", colon);
                field = &line[0..colon];
                value = &line[colon+1..];
            }
            attrs.insert(field, value);
            println!("> Front field: '{}', value: '{}'", field, value);
        } else {
            println!("> What is this: '{}'", line);
        }
    }

    post.attrs = attrs;
    post.lines = lines;
}
