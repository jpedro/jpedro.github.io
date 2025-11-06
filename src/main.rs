use std::fs;
use std::path::Path;

use clap::Parser;

use crate::post::Post;

mod args;
mod find;
mod post;

const SLASH: &str = "/";

fn main() {
    let args = args::Args::parse();
    println!("- Dir   {:?}", args.dir);

    let found = find::files(Path::new(&args.dir));
    if let Ok(paths) = found {
        for path in paths {
            let p = post::load(&path).expect("Failed");
            println!("");
            println!("- Path  {:?}", p.path);
            println!("- Text  {:?}", p.text);
            println!("- Html  {:?}", p.html);
            println!("- Title {:?}", p.title);
            // println!("- Lines {:?}", post.lines);
            println!("- Attrs {:?}", p.attrs);
            let docs = dest(&args.dir, &p);
            let parent = Path::new(&docs).parent().unwrap();
            println!("- Docs  {:?}", docs);
            println!("- Parent  {:?}", parent);
            let _ = fs::create_dir_all(parent);
            let d = Path::new(&docs);
            post::render(&p, &d);
        }
    }
}

fn dest(dir: &str, p: &Post) -> String {
    format!(
        "docs/{}.html",
        p.path.
            replace(".md", "").
            replace(&dir, "").
            strip_prefix(SLASH)
            .unwrap()
    )
}
