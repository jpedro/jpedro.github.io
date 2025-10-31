use std::fs;
use std::io::Error;
use std::path::Path;

use std::collections::HashMap;

const TAG_H1: &str = "# ";

#[allow(dead_code)]
pub struct Post<'a> {
    pub path: &'a Path,
    pub text: String,
    pub lines: Vec<String>,
    pub title: String,
    pub attrs: HashMap<String, String>,
}

pub fn process(path: &Path) -> Result<Post<'_>, Error> {
    let text = fs::read_to_string(path).expect("Couldn't read file");

    // println!("{:?}:\n---\n{}===\n", path, &text);

    let mut post = Post {
        path: &path,
        text: text,
        title: path
            .file_name()
            .expect("Failed to get file name")
            .to_string_lossy()
            .to_string(),
        lines: vec![],
        attrs: HashMap::new(),
    };
    parse(&mut post, &path);

    Ok(post)
}

pub fn parse(_post: &mut Post, path: &Path) {
    let mut _attrs = HashMap::<String, String>::new();
    let mut found = false;

    for line in fs::read_to_string(path).unwrap().lines() {
        if !found && line.starts_with(TAG_H1) {
            println!("> xxxxx --- Found the H1 tag");
            found = true;
        }
        println!("> {}", line);
    }
}
