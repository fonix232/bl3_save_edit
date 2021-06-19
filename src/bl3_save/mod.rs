use std::fmt;
use std::fmt::Formatter;

use anyhow::{bail, Result};
use nom::Finish;

use crate::bl3_save::character_data::CharacterData;
use crate::parser::{
    decrypt, read_custom_format_data, read_header, read_int, read_short, read_str,
};

mod character_data;
mod player_class;
mod util;

#[derive(Debug)]
pub struct CustomFormatData {
    pub guid: String,
    pub entry: u32,
}

#[derive(Debug)]
pub struct Bl3Save {
    save_game_version: u32,
    package_version: u32,
    engine_major: u16,
    engine_minor: u16,
    engine_patch: u16,
    engine_build: u32,
    build_id: String,
    custom_format_version: u32,
    custom_format_data_count: u32,
    custom_format_data: Vec<CustomFormatData>,
    save_game_type: String,
    character_data: CharacterData,
}

impl Bl3Save {
    pub fn from_data(data: &mut [u8]) -> Result<Self> {
        let (r, _) = read_header(data).finish()?;
        let (r, save_game_version) = read_int(r).finish()?;
        let (r, package_version) = read_int(r).finish()?;
        let (r, engine_major) = read_short(r).finish()?;
        let (r, engine_minor) = read_short(r).finish()?;
        let (r, engine_patch) = read_short(r).finish()?;
        let (r, engine_build) = read_int(r).finish()?;
        let (r, build_id) = read_str(r).finish()?;
        let (r, custom_format_version) = read_int(r).finish()?;
        let (r, custom_format_data_count) = read_int(r).finish()?;
        let (r, custom_format_data) =
            read_custom_format_data(r, custom_format_data_count).finish()?;
        let (r, save_game_type) = read_str(r).finish()?;
        let (r, remaining_data_len) = read_int(r).finish()?;

        let data_read = data.len() - r.len();

        let mut remaining_data = &mut data[data_read..];

        if remaining_data.len() != remaining_data_len as usize {
            bail!("failed to parse the remaining save file data - failed to parse the first part of the save file");
        }

        let character = decrypt(&mut remaining_data)?;

        let character_data = CharacterData::from_character(character)?;

        Ok(Self {
            save_game_version,
            package_version,
            engine_major,
            engine_minor,
            engine_patch,
            engine_build,
            build_id,
            custom_format_version,
            custom_format_data_count,
            custom_format_data,
            save_game_type,
            character_data,
        })
    }
}

impl fmt::Display for Bl3Save {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // writeln!(f, "Savegame version: {}", self.save_game_version)?;
        // writeln!(f, "Package version: {}", self.package_version)?;
        // writeln!(
        //     f,
        //     "Engine version: {}.{}.{}.{}",
        //     self.engine_major, self.engine_minor, self.engine_patch, self.engine_build
        // )?;
        // writeln!(f, "Build ID: {}", self.build_id)?;
        // writeln!(f, "Custom Format Version: {}", self.custom_format_version)?;
        // writeln!(
        //     f,
        //     "Custom Format Data Count: {}",
        //     self.custom_format_data_count
        // )?;
        // writeln!(f, "Savegame type: {}", self.save_game_type)
        writeln!(
            f,
            "Character: {}",
            self.character_data.character.preferred_character_name
        )?;
        writeln!(
            f,
            "Savegame ID: {}",
            self.character_data.character.save_game_id
        )?;
        writeln!(
            f,
            "Savegame GUID: {}",
            self.character_data.character.save_game_guid
        )?;
        writeln!(f, "Player Class: {}", self.character_data.player_class)?;
        writeln!(f, "XP: {}", self.character_data.character.experience_points)?;
        writeln!(f, "Level: {}", self.character_data.player_level)?;
        writeln!(f, "Guardian Rank: {}", self.character_data.guardian_rank)?;
        writeln!(f, "Money: {}", self.character_data.money)?;
        writeln!(f, "Eridium: {}", self.character_data.eridium)?;
        writeln!(
            f,
            "Playthroughs Completed: {}",
            self.character_data.character.playthroughs_completed
        )
    }
}
