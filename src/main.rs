use std::path::Path;
use std::path::PathBuf;
use clap::Parser;

mod args;
mod find;
mod post;

fn main() {
    let args = args::Args::parse();
    println!("- Dir   {:?}", args.dir);

    let found = find::files(PathBuf::from(&args.dir));
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
            let dest = Path::new("/tmp/foo.html");
            post::render(&p, &dest);
        }
    }
}
