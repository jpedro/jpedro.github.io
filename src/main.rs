use std::path::PathBuf;
use clap::Parser;

mod cli;
mod find;
mod post;

fn main() {
    let args = cli::Args::parse();
    println!("- Dir   {:?}", args.dir);

    let found = find::files(PathBuf::from(&args.dir));
    if let Ok(paths) = found {
        for path in paths {
            let post = post::process(&path).expect("Failed");
            println!("");
            println!("- Path  {:?}", post.path);
            println!("- Text  {:?}", post.text);
            println!("- Title {:?}", post.title);
            println!("- Lines {:?}", post.lines);
            println!("- Attrs {:?}", post.attrs);
        }
    }
}
