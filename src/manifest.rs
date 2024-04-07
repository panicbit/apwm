use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    worlds: BTreeMap<String, Version>,
}

impl Manifest {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let data = fs::read_to_string(path).context("failed to read installed world metadata")?;
        let data = toml::from_str::<Self>(&data)?;

        Ok(data)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let data = toml::to_string_pretty(self)?;

        fs::write(path, data)?;

        Ok(())
    }
}
