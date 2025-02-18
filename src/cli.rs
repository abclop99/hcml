//! The command line interface

use std::path::PathBuf;

use clap::Parser;

/// A program to convert HCML to HTML
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// The file to convert
    pub file: PathBuf,
}
