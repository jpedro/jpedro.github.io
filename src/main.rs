use std::path::PathBuf;

mod args;

fn main() {
    let values = args::Args::parse();
    let files = args::Files::new();
    // values.load();
    // values::parse();

    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(error) => panic!("Couldn't use currrent dir: {}", error),
    };
    println!("CWD: {}", cwd);
    let dir = PathBuf::from(values.dir);
    println!("DIR: {:?}", dir);

    println!("{:?}", files.find(path:: ".posts".));

    for file in files.files {
        println!("- {:?}.", file);
    }

    for _ in 0 .. values.count {
        println!("{} {}.",  values.prefix, values.name);
    }
}
