use crate::utils;
use crate::utils::types;
use std::fs;
use std::io;

use serde_json;

pub trait ToFile {
    fn to_file(&self, _: &std::path::Path) -> io::Result<()>
    where
        Self: Sized;
}

pub trait FromFile {
    fn from_file(_: &std::path::Path) -> io::Result<Self>
    where
        Self: Sized;
}

impl ToFile for types::WordPrevalenceMap {
    fn to_file(&self, ofile: &std::path::Path) -> io::Result<()> {
        let content = serde_json::to_string(self)?;
        fs::write(&ofile, content)
    }
}

impl FromFile for types::WordPrevalenceMap {
    fn from_file(ofile: &std::path::Path) -> io::Result<types::WordPrevalenceMap> {
        let content = fs::read_to_string(ofile)?;
        let parse = serde_json::from_str::<types::WordPrevalenceMap>(&content);
        parse.map_err(utils::make_io_error)
    }
}
