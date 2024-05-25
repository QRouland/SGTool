#![warn(missing_docs)]
//! sgtool is a library for analyze of the replay of the Stormgate game.
//!
//! 
//! # Example
//! ```
//! let replay_path = "replay.SGReplay";
//! let mut buffer : Vec<u8> = vec![]; 
//! let replay : Replay = Replay::load_file(replay_path, &mut buffer)?;
//! ```
//! 
pub mod parser;
mod protos;