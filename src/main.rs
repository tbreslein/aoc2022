mod common;
mod solutions;

use clap::Parser;
use std::error::Error;

/// Run the Advent of Code 2021 challenges
#[derive(Parser)]
#[clap(version = "0.0.1", author = "Author: github.com/tbreslein")]
struct Opts {
    /// which day of the AoC you wanna run; valid values: {1, 2, ..}
    #[clap(short, long)]
    day: i32,

    /// which part of that day you wanna run; valid values: {1, 2}
    #[clap(short, long)]
    part: i32,

    /// file containing the data relevant to the challenge
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    common::pick_challenge::pick_challenge(opts.day, opts.part, opts.file)?;
    Ok(())
}
