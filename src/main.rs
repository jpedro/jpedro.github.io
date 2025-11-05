use std::fs;
use std::path::Path;
// use std::path::PathBuf;
use clap::Parser;

mod args;
mod find;
mod post;

fn main() {
    let args = args::Args::parse();
    let slash = "/";
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
            let docs = format!("docs/{}.html", p.path.replace(".md", "").replace(&args.dir, "").strip_prefix(slash).unwrap());
            let parent = Path::new(&docs).parent().unwrap();
            println!("- Docs  {:?}", docs);
            println!("- Parent  {:?}", parent);
            let _ = fs::create_dir_all(parent);
            let dest = Path::new(&docs);
            post::render(&p, &dest);
        }
    }
}
