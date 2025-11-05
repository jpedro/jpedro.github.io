use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory to load files from
    #[arg(short, long, default_value = ".posts")]
    pub dir: String,
}
