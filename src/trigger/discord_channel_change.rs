use dirs;

use crate::trigger::Trigger;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

pub struct DiscordChannelChecker;

impl Trigger<String, io::Error> for DiscordChannelChecker {
    fn check(&mut self) -> Result<String, io::Error> {
        let mut channel = String::new();
        let mut path = match dirs::home_dir() {
            Some(path) => path,
            None => return Err(io::Error::from(io::ErrorKind::NotFound)),
        };
        path.push(PathBuf::from("Dev/.is_in_discord"));
        File::open(path)?.read_to_string(&mut channel)?;
        Ok(channel)
    }
}
