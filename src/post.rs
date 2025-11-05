use std::fs;
use std::io::Error;
use std::path::Path;

use std::collections::HashMap;

// const TAG_H1: &str = "# ";

pub struct Post<'a> {
    pub path: &'a Path,
    pub text: String,
    pub lines: Vec<String>,
    pub title: &'a str,
    pub attrs: HashMap<String, String>,
}

pub fn process(path: &Path) -> Result<Post<'_>, Error> {
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

    let post = Post {
        path: &path,
        text: "text.to_owned()".to_owned(),
        title: &path.file_name().unwrap().to_str().unwrap(),
        lines: read_lines(&path),
        attrs: read_attrs(&path),
    };

    Ok(post)
}

fn read_lines(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line: &str| line.into())
        .collect()
}

fn read_attrs(path: &Path) -> HashMap<String, String> {
    let mut attrs: HashMap::<String, String> = HashMap::new();
    let mut _found = false;
    for line in fs::read_to_string(path).unwrap().lines() {
        let line = line.to_owned();
        if !line.starts_with("<!--") {
            continue;
        }

        let front = line.replace("<!--", "").replace("-->", "");
        println!("> Found front: '{}'", front);
        let mut field = line.clone();
        let mut value = "true".to_string();
        if let Some(colon) = line.find(":") {
            println!("> Found colon: '{}'", colon);
            field = line[0..colon].to_string();
            value = line[colon+1..].to_string();
        }
        attrs.insert(field.clone(), value.to_string());
        println!("> Front field: '{}', value: '{}'", &field, &value);
    }
    attrs
}
