//! sgtool is a tool for analyze of the replay of the Stormgate game.
//!
//! 
//! # Example
//! ```
//! sgtool -i replays/replay.SGReplay
//! ```

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::path::PathBuf;
use clap::Parser;
use crate::parser::Replay;


mod protos;
mod parser;

/// Simple replay Stormgate replay parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of replay to parse
    #[arg(short, long)]
    input: PathBuf,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    info!("Input : {}", args.input.to_string_lossy());
    let mut bytes = Vec::new();
    debug!("{:#?}", Replay::load_file(&args.input, &mut bytes).unwrap());
}