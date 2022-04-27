use std::io::Error;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

pub struct Resource<'a> {
    pub path: &'a Path,
}

impl<'a> Resource<'a> {

    pub fn folder_contents(self) -> Result<Vec<PathBuf>, Error> {
        Ok(fs::read_dir(self.path)?
            .into_iter()
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap().path())
            .collect())
    }
}