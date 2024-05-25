// Automatically generated rust module for 'stormgate.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LeaveReason {
    Unknown = 0,
    Surrender = 1,
    Leave = 2,
}

impl Default for LeaveReason {
    fn default() -> Self {
        LeaveReason::Unknown
    }
}

impl From<i32> for LeaveReason {
    fn from(i: i32) -> Self {
        match i {
            0 => LeaveReason::Unknown,
            1 => LeaveReason::Surrender,
            2 => LeaveReason::Leave,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for LeaveReason {
    fn from(s: &'a str) -> Self {
        match s {
            "Unknown" => LeaveReason::Unknown,
            "Surrender" => LeaveReason::Surrender,
            "Leave" => LeaveReason::Leave,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayChunk<'a> {
    pub timestamp: i32,
    pub client_id: i32,
    pub inner: Option<stormgate::mod_ReplayChunk::Wrapper<'a>>,
}

impl<'a> MessageRead<'a> for ReplayChunk<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.timestamp = r.read_int32(bytes)?,
                Ok(16) => msg.client_id = r.read_int32(bytes)?,
                Ok(26) => msg.inner = Some(r.read_message::<stormgate::mod_ReplayChunk::Wrapper>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ReplayChunk<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.timestamp == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.client_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.client_id) as u64) }
        + self.inner.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.timestamp != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.timestamp))?; }
        if self.client_id != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.client_id))?; }
        if let Some(ref s) = self.inner { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_ReplayChunk {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Wrapper<'a> {
    pub content: Option<stormgate::mod_ReplayChunk::mod_Wrapper::ReplayContent<'a>>,
}

impl<'a> MessageRead<'a> for Wrapper<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.content = Some(r.read_message::<stormgate::mod_ReplayChunk::mod_Wrapper::ReplayContent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Wrapper<'a> {
    fn get_size(&self) -> usize {
        0
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.content { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Wrapper {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayContent<'a> {
    pub content_type: stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type<'a>,
}

impl<'a> MessageRead<'a> for ReplayContent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.content_type = stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::map(r.read_message::<stormgate::Map>(bytes)?),
                Ok(98) => msg.content_type = stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::player(r.read_message::<stormgate::Player>(bytes)?),
                Ok(106) => msg.content_type = stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::change_slot(r.read_message::<stormgate::LobbyChangeSlot>(bytes)?),
                Ok(122) => msg.content_type = stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::set_variable(r.read_message::<stormgate::LobbySetVariable>(bytes)?),
                Ok(146) => msg.content_type = stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::start_game(r.read_message::<stormgate::StartGame>(bytes)?),
                Ok(202) => msg.content_type = stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::player_left_game(r.read_message::<stormgate::PlayerLeftGame>(bytes)?),
                Ok(298) => msg.content_type = stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::assign_player_slot(r.read_message::<stormgate::AssignPlayerSlot>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ReplayContent<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.content_type {
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::map(ref m) => 1 + sizeof_len((m).get_size()),
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::player(ref m) => 1 + sizeof_len((m).get_size()),
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::change_slot(ref m) => 1 + sizeof_len((m).get_size()),
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::set_variable(ref m) => 1 + sizeof_len((m).get_size()),
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::start_game(ref m) => 2 + sizeof_len((m).get_size()),
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::player_left_game(ref m) => 2 + sizeof_len((m).get_size()),
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::assign_player_slot(ref m) => 2 + sizeof_len((m).get_size()),
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.content_type {            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::map(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::player(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::change_slot(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::set_variable(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::start_game(ref m) => { w.write_with_tag(146, |w| w.write_message(m))? },
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::player_left_game(ref m) => { w.write_with_tag(202, |w| w.write_message(m))? },
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::assign_player_slot(ref m) => { w.write_with_tag(298, |w| w.write_message(m))? },
            stormgate::mod_ReplayChunk::mod_Wrapper::mod_ReplayContent::OneOfcontent_type::None => {},
    }        Ok(())
    }
}

pub mod mod_ReplayContent {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfcontent_type<'a> {
    map(stormgate::Map<'a>),
    player(stormgate::Player<'a>),
    change_slot(stormgate::LobbyChangeSlot),
    set_variable(stormgate::LobbySetVariable),
    start_game(stormgate::StartGame),
    player_left_game(stormgate::PlayerLeftGame),
    assign_player_slot(stormgate::AssignPlayerSlot<'a>),
    None,
}

impl<'a> Default for OneOfcontent_type<'a> {
    fn default() -> Self {
        OneOfcontent_type::None
    }
}

}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Map<'a> {
    pub folder: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub seed: i32,
}

impl<'a> MessageRead<'a> for Map<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.folder = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.seed = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Map<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.folder == "" { 0 } else { 1 + sizeof_len((&self.folder).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.seed == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seed) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.folder != "" { w.write_with_tag(10, |w| w.write_string(&**&self.folder))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.seed != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.seed))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Player<'a> {
    pub uuid: Option<stormgate::UUID>,
    pub name: Option<stormgate::mod_Player::PlayerName<'a>>,
}

impl<'a> MessageRead<'a> for Player<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.uuid = Some(r.read_message::<stormgate::UUID>(bytes)?),
                Ok(26) => msg.name = Some(r.read_message::<stormgate::mod_Player::PlayerName>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Player<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uuid { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Player {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerName<'a> {
    pub nickname: Cow<'a, str>,
    pub discriminator: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for PlayerName<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.nickname = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.discriminator = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PlayerName<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.nickname == "" { 0 } else { 1 + sizeof_len((&self.nickname).len()) }
        + if self.discriminator == "" { 0 } else { 1 + sizeof_len((&self.discriminator).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.nickname != "" { w.write_with_tag(10, |w| w.write_string(&**&self.nickname))?; }
        if self.discriminator != "" { w.write_with_tag(18, |w| w.write_string(&**&self.discriminator))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LobbyChangeSlot {
    pub choice: Option<stormgate::mod_LobbyChangeSlot::SlotChoice>,
}

impl<'a> MessageRead<'a> for LobbyChangeSlot {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.choice = Some(r.read_message::<stormgate::mod_LobbyChangeSlot::SlotChoice>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LobbyChangeSlot {
    fn get_size(&self) -> usize {
        0
        + self.choice.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.choice { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_LobbyChangeSlot {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SlotChoice {
    pub choice_type: stormgate::mod_LobbyChangeSlot::mod_SlotChoice::OneOfchoice_type,
}

impl<'a> MessageRead<'a> for SlotChoice {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.choice_type = stormgate::mod_LobbyChangeSlot::mod_SlotChoice::OneOfchoice_type::specific_slot(r.read_message::<stormgate::mod_LobbyChangeSlot::mod_SlotChoice::SpecificSlot>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SlotChoice {
    fn get_size(&self) -> usize {
        0
        + match self.choice_type {
            stormgate::mod_LobbyChangeSlot::mod_SlotChoice::OneOfchoice_type::specific_slot(ref m) => 1 + sizeof_len((m).get_size()),
            stormgate::mod_LobbyChangeSlot::mod_SlotChoice::OneOfchoice_type::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.choice_type {            stormgate::mod_LobbyChangeSlot::mod_SlotChoice::OneOfchoice_type::specific_slot(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            stormgate::mod_LobbyChangeSlot::mod_SlotChoice::OneOfchoice_type::None => {},
    }        Ok(())
    }
}

pub mod mod_SlotChoice {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SpecificSlot {
    pub slot: i32,
}

impl<'a> MessageRead<'a> for SpecificSlot {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.slot = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SpecificSlot {
    fn get_size(&self) -> usize {
        0
        + if self.slot == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.slot) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.slot != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.slot))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfchoice_type {
    specific_slot(stormgate::mod_LobbyChangeSlot::mod_SlotChoice::SpecificSlot),
    None,
}

impl Default for OneOfchoice_type {
    fn default() -> Self {
        OneOfchoice_type::None
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LobbySetVariable {
    pub slot: i32,
    pub variable_id: u32,
    pub value: u32,
}

impl<'a> MessageRead<'a> for LobbySetVariable {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(24) => msg.slot = r.read_int32(bytes)?,
                Ok(32) => msg.variable_id = r.read_uint32(bytes)?,
                Ok(40) => msg.value = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LobbySetVariable {
    fn get_size(&self) -> usize {
        0
        + if self.slot == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.slot) as u64) }
        + if self.variable_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.variable_id) as u64) }
        + if self.value == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.slot != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.slot))?; }
        if self.variable_id != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.variable_id))?; }
        if self.value != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StartGame { }

impl<'a> MessageRead<'a> for StartGame {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for StartGame { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerLeftGame {
    pub player_uuid: Option<stormgate::UUID>,
    pub reason: stormgate::LeaveReason,
}

impl<'a> MessageRead<'a> for PlayerLeftGame {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.player_uuid = Some(r.read_message::<stormgate::UUID>(bytes)?),
                Ok(16) => msg.reason = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerLeftGame {
    fn get_size(&self) -> usize {
        0
        + self.player_uuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.reason == stormgate::LeaveReason::Unknown { 0 } else { 1 + sizeof_varint(*(&self.reason) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player_uuid { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.reason != stormgate::LeaveReason::Unknown { w.write_with_tag(16, |w| w.write_enum(*&self.reason as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AssignPlayerSlot<'a> {
    pub uuid: Option<stormgate::UUID>,
    pub slot: i64,
    pub nickname: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for AssignPlayerSlot<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.uuid = Some(r.read_message::<stormgate::UUID>(bytes)?),
                Ok(16) => msg.slot = r.read_int64(bytes)?,
                Ok(26) => msg.nickname = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AssignPlayerSlot<'a> {
    fn get_size(&self) -> usize {
        0
        + self.uuid.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.slot == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.slot) as u64) }
        + if self.nickname == "" { 0 } else { 1 + sizeof_len((&self.nickname).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.uuid { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.slot != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.slot))?; }
        if self.nickname != "" { w.write_with_tag(26, |w| w.write_string(&**&self.nickname))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UUID {
    pub part1: i64,
    pub part2: i64,
}

impl<'a> MessageRead<'a> for UUID {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.part1 = r.read_int64(bytes)?,
                Ok(16) => msg.part2 = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UUID {
    fn get_size(&self) -> usize {
        0
        + if self.part1 == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.part1) as u64) }
        + if self.part2 == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.part2) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.part1 != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.part1))?; }
        if self.part2 != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.part2))?; }
        Ok(())
    }
}

