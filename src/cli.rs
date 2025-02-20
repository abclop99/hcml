//! The command line interface

use std::path::PathBuf;

use clap::Parser;

/// A program to convert HCML to HTML
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// The file to convert.
    /// Use `-` to read from stdin.
    pub file: PathBuf,
}
