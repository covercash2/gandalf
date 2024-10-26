use std::path::Path;

use crate::error::{Error, Result};

pub fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    std::fs::read_to_string(path).map_err(Error::FileRead)
}
