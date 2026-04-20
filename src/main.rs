mod args;
mod find;
mod posts;

use std::fs;
use std::path::Path;
use clap::Parser;
use posts::Post;

const SLASH: &str = "/";

fn main() {
    let args = args::Args::parse();
    println!("- Dir   {:?}", args.dir);

    let found = find::files(Path::new(&args.dir));
    let Ok(paths) = found else {
        println!("Found no posts");
        return;
    };

    for path in paths {
        let p = posts::load(&path).expect("Failed");
        println!("");
        println!("- Path  {:?}", p.path);
        println!("- Title {:?}", p.title);
        println!("- Attrs {:?}", p.attrs);
        // println!("- Text  {:?}", p.text);
        // println!("- Html  {:?}", p.html);
        // println!("- Lines {:?}", post.lines);
        let docs = dest(&args.dir, &p);
        let parent = Path::new(&docs).parent().unwrap();
        // println!("- Docs  {:?}", docs);
        // println!("- Parent  {:?}", parent);
        let _ = fs::create_dir_all(parent);
        let d = Path::new(&docs);
        posts::render(&p, &d);
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
