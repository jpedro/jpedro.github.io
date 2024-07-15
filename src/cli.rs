use std::fs;
use std::io::Error;
use std::path::PathBuf;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory to load files from
    #[arg(short, long, default_value = ".posts")]
    pub dir: String,

    /// Greeting prefix
    #[arg(short, long, default_value = "Hello")]
    pub prefix: String,

    /// Number of times to greet
    #[arg(short, long)]
    pub name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

impl Args {

}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Files {
    pub files: Vec<PathBuf>,
}

impl Files {
    // #[allow(dead_code)]
    pub fn find(dir: &str) -> Result<Vec<PathBuf>, Error> {
        let dir = PathBuf::from(dir);
        let mut paths = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            paths.push(path);
        }

        Ok(paths)
    }
}
