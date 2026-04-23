use std::fs;
use std::path::Path;
use clap::Parser;

mod args;
mod finder;
mod posts;
mod render;

fn main() {
    let args = args::Args::parse();
    println!("Using dir {}", &args.dir);

    let files = finder::files(Path::new(&args.dir));
    let Ok(paths) = files else {
        println!("Found no posts inside {}", &args.dir);
        return;
    };

    let renderer = render::Renderer::new("src/templates/**/*.html".into());

    for path in paths {
        let post = posts::load(&path).expect("Failed to load post");
        println!("---");
        println!("  Path  {:?}", post.path);
        println!("  Title {:?}", post.title);
        println!("  Attrs {:?}", post.attrs);
        let dest = post.dest(&args.dir);
        let root = Path::new(&dest).parent().unwrap();
        let _ = fs::create_dir_all(&root);
        let file = Path::new(&dest);
        post.render(&renderer, &file);
    }
}
