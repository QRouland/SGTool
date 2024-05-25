//! A parser for Stormgate Replay.
//!
//! 
//! # Example
//! ```
//! let replay_path = "replay.SGReplay";
//! let mut buffer : Vec<u8> = vec![]; 
//! let replay : Replay = Replay::load_file(replay_path, &mut buffer)?;
//! ```

use std::{fs::OpenOptions, io::{BufRead, BufReader, Read}, path::Path};
use byteorder::{ByteOrder, LittleEndian};
use flate2::bufread::GzDecoder;
use quick_protobuf::BytesReader;

use crate::protos::stormgate::ReplayChunk;

/// Stormgate Replay Header.
//
// It consists of 16 bytes at the top of the replay
#[derive(Debug, Clone)]
pub struct Header {
    h1: u32, // 12 first bytes repsentation are unknown at the moment
    build_number: u32, // 4 next is he build number
}

/// Stormgate Replay Payload.
///
// After the the 16 bytes header, the actual payload of the replay of Protobuf messages that are gzipped
// Each Protobuf messages represents events that appended in the game from the differetns clients
#[derive(Debug, Clone)]
pub struct Payload<'a> {
    chunks: Vec<ReplayChunk<'a>>,
}

/// Stormgate Replay.
#[derive(Debug, Clone)]
pub struct Replay<'a> {
    header: Header, // header of the replay
    payload: Payload<'a>, // the content of a replay is set of messages
}

/// Load n bytes into a buffer.
fn load_part<'a, R: Read>(reader: &'a mut R) -> impl FnMut(usize) -> Result<Vec<u8>, std::io::Error> + 'a {
    move |size| {
        let mut buf = vec![0u8; size];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Header {
    /// Load the 16 bytes as a `Header`.
    fn load<T: BufRead>(reader: &mut T) -> Result<Header, std::io::Error> {
        let mut load = load_part(reader);
        let h1 = LittleEndian::read_u32(&load(12)?);
        let build_number = LittleEndian::read_u32(&load(4)?);
        Ok(Header { h1, build_number })
    }
}

impl<'a> Payload<'a> {
    /// Load the replay content as a `Payload`.
    fn load<T: BufRead>(buf_reader: &mut T, buf: &'a mut Vec<u8>) -> Result<Payload<'a>, Box<dyn std::error::Error>> {
        let mut d = GzDecoder::new(buf_reader);
        d.read_to_end(buf)?;
        let mut reader = BytesReader::from_bytes(&buf);
        
        let mut data = Vec::new();

        while !reader.is_eof() {
            let read_message = reader.read_message::<ReplayChunk>(buf)?;
            data.push(read_message);
        }

        Ok(Payload { chunks: data })
    }
}


impl<'a> Replay<'a> {
    
    fn load<T: BufRead>(buf_reader: &mut T, buf: &'a mut Vec<u8>) -> Result<Replay<'a>, Box<dyn std::error::Error>> {
        // Get Header
        let header = Header::load(buf_reader)?;

        // Get Payload
        let data = Payload::load(buf_reader, buf)?;
        
        Ok(Replay { header, payload: data })
    }

    /// Load a replay file as `Replay`.
    /// 
    /// # Arguments
    /// * `path` : Path of the storgmate replay file.
    /// * `buf` : Byte Buffer use by the parser.
    /// 
    /// # Example
    /// ```
    /// let replay_path = "replay.SGReplay";
    /// let mut buffer : Vec<u8> = vec![]; 
    /// let replay : Replay = Replay::load_file(replay_path, &mut buffer)?;
    /// ```
    pub fn load_file(path: &Path, buf: &'a mut Vec<u8>) -> Result<Replay<'a>, Box<dyn std::error::Error>> {
        let file = OpenOptions::new().read(true).open(path).unwrap();
        let mut buf_reader = BufReader::new(file);
        Replay::load(&mut buf_reader, buf)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file() {
        // Replay used is form Alpha phase of stormgate, threfore is not provided in the repo to avoid any problem with NDA at this time 
        // TODO : when the game officially came out fix provide sample replay for testing directly in the repo
        let replay_path = Path::new("replays/CL55366-2024.05.12-01.53.SGReplay"); 
        let mut buffer : Vec<u8> = vec![];
        let r = Replay::load_file(replay_path, &mut buffer).unwrap();
        assert_eq!(r.header.build_number, 55366);
        assert_eq!(r.payload.chunks.len(), 1373);
    }
}

