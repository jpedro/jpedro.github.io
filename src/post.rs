use std::fs;
use std::path::Path;


#[allow(dead_code)]
pub fn process(path: &Path) {
    let text = fs::read_to_string(path)
        .expect("Couldn't read file");

    println!("{:?}:\n---\n{}===\n", path, text);
}
