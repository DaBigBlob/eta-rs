use std::{fs, path::PathBuf, process::ExitCode};

use clap::Parser;
use eta_core::basic;

const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\n",
    "Authors: ", env!("CARGO_PKG_AUTHORS"), "\n",
    "License: ", env!("CARGO_PKG_LICENSE"),
);

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    author,
    version = VERSION,
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Cli {
    /// Treat INPUT as a path to a file and read the S-pair from it
    #[arg(short, long, value_name = "PATH", conflicts_with = "input")]
    file: Option<PathBuf>,

    /// Literal S-pair input (unless --file is used)
    #[arg(value_name = "S-PAIR", required_unless_present = "file")]
    input: Option<String>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let input = match cli.file {
        Some(path) => match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("eta-cli: failed to read '{}': {}", path.display(), e);
                return ExitCode::from(2);
            }
        },
        None => cli.input.unwrap_or_default(),
    };

    let mut out = String::new();
    basic::execute(&mut out, input.chars().into_iter());
    print!("{out}");

    ExitCode::SUCCESS
}
