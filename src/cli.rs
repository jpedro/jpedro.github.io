use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // /// Greeting prefix
    // #[arg(short, long, default_value = "Hello")]
    // pub prefix: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value = "posts")]
    // pub name: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // pub count: u8,

    /// Directory to load files from
    #[arg(short, long, default_value = ".test")]
    pub dir: String,
}

// impl Args {
// }
