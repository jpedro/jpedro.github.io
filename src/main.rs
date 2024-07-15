use clap::Parser;

mod cli;

fn main() {
    let args = cli::Args::parse();
    println!("DIR: {}", args.dir);

    for _ in 0 .. args.count {
        println!("{} {}!", args.prefix, args.name);
    }

    let found = cli::Files::find(&args.dir);
    // println!("{:?}", files);
    if let Ok(files) = found {
        for file in files {
            println!("- {:?}.", &file);
        }
    } else {
        println!("No files found.");
    }
}
