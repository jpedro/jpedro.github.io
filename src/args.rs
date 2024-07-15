// use std::fs;
// use std::path::PathBuf;

use clap::Parser;
// use clap::Error;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(short, long, default_value=".posts")]
    pub dir String;

    #[arg(short, long)]
    pub name: String,

    #[arg(long, default_value="Hello")]
    pub prefix: String,

    #[arg(short, long, default_value_t=1)]
    pub count: u8,
}

// #[derive(Debug)]
// pub struct Files {
//     pub files: Vec<PathBuf>,
// }

// impl Files {
//     pub fn new() -> Self {
//         Self {
//             files: Vec::new(),
//         }
//     }

//     pub fn find(&mut self, dir: &PathBuf) -> Result<u8, Error> {
//         let mut count: u8 = 0;
//         for entry in fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();
//             self.files.push(path);
//             count += 1;
//         }

//         Ok(count)
//     }
// }
