use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

pub mod v1;

pub use v1::*;

#[derive(Deserialize, Debug)]
#[serde(tag = "version")]
pub enum VersionedIndex {
    #[serde(rename = "1")]
    V1(v1::Index),
}

pub fn load(path: impl AsRef<Path>) -> Result<Index> {
    let index = fs::read_to_string(path).context("failed to read index")?;
    let index = toml::from_str::<VersionedIndex>(&index).context("failed to parse index")?;
    let VersionedIndex::V1(index) = index;

    Ok(index)
}
