use clap::Parser;
use std::{num::NonZeroUsize, path::PathBuf};

/// A Brainfuck interpreter written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// The path to Brainfuck source file.
    pub(crate) program: PathBuf,

    /// Number of cells in Brainfuck virtual machine memory.
    #[arg(short, long, default_value = "30000")]
    pub(crate) cells: NonZeroUsize,

    /// Allow virtual machine memory to auto extend or not.
    #[arg(short, long)]
    pub(crate) extensible: bool,
}
