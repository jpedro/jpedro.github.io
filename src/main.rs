use std::path::PathBuf;

use clap::Parser;

mod cli;
mod find;
mod post;


fn main() {
    let args = cli::Args::parse();
    println!("DIR: {}", args.dir);

    for _ in 0 .. args.count {
        println!("{} {}!", args.prefix, args.name);
    }

    let found = find::files(PathBuf::from(&args.dir));
    if let Ok(paths) = found {
        for path in paths {
            // println!("- {:?}.", path);
            post::process(&path);
        }
    }
}
