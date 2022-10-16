use clap::Parser;
use clap::{Subcommand};

use crate::chapter1::chapter1::{Chapter1Args,chapter1_command_runner};

mod chapter1;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[clap(
    name = "Bioinformatics Algorithms: An active learning guide",
    version = "1.0.0",
    author = "Inura De Zoysa, github: inuradz",
    about = "Stores all the examples in an easy to access way"
)]
struct Cli {
    /// The pattern to look for
    #[clap(subcommand)]
    chapter: Chapters,
}

#[derive(Debug, Subcommand)]
enum Chapters {
    Chapter1(Chapter1Args),
}


fn main() {
    let args = Cli::parse();
    match args.chapter {
        Chapters::Chapter1(chapter1)  => 
        chapter1_command_runner(chapter1)
    }
}
