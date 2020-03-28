use crate::Trigger;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct ReadFileContents<'a> {
    file_path: &'a Path,
}

impl<'a> ReadFileContents<'a> {
    pub fn with_path<T: AsRef<Path> + ?Sized>(path: &'a T) -> Self {
        Self {
            file_path: path.as_ref(),
        }
    }
}

impl<'a> Trigger<String, io::Error> for ReadFileContents<'a> {
    fn check(&mut self) -> Result<String, io::Error> {
        let mut text = String::new();
        File::open(&self.file_path)?.read_to_string(&mut text)?;
        Ok(text)
    }
}
