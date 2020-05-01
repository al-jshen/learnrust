#![allow(unused)]

use structopt::StructOpt;

/// Search for a pattern in file and display line(s) containing it.
#[derive(StructOpt)]
struct Cli {
    /// Pattern to look for
    pattern: String,
    /// Path to file to search
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
}
