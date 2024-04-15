use anyhow::{ensure, Context, Result};

use std::fs::File;
use std::io::{stderr, stdout, BufReader, Read, Write};
use std::path::Path;

use clap::{arg, Command};

use textractor::extraction::extract;

pub mod consts {
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
}

fn build_parser() -> Command {
    Command::new("textractor")
        .bin_name("textractor")
        .version(consts::VERSION)
        .author("Nathan LeRoy")
        .about("A command line tool that extracts text from files.")
        .arg(arg!(<path> "Path to file").required(true))
}

fn main() -> Result<()> {
    // parse the cli
    let app = build_parser();
    let matches = app.get_matches();

    // build handler for stdout
    let stdout = stdout();
    let mut stdout_handle = stdout.lock();
    let mut stderr_handle = stderr();

    let file_path = matches
        .get_one::<String>("path")
        .with_context(|| "Failed to get path argument")?;

    let file_path = Path::new(&file_path);

    ensure!(file_path.exists(), "File does not exist");

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut data = Vec::new();

    reader.read_to_end(&mut data)?;

    let text = extract(&data)?;

    match text {
        Some(text) => {
            stdout_handle.write_all(text.as_bytes())?;
        }
        None => {
            stderr_handle.write_all("Unsupported file type".as_bytes())?;
        }
    }

    Ok(())
}
