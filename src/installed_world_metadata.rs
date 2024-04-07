use std::path::Path;
use std::{fs, io};

use anyhow::{Context, Result};
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::index::{Release, World};

#[derive(Deserialize, Serialize, Debug)]
pub struct InstalledWorldMetadata {
    pub name: String,
    pub version: Version,
    pub url: String,
}

impl From<(&World, &Release)> for InstalledWorldMetadata {
    fn from((world, release): (&World, &Release)) -> Self {
        InstalledWorldMetadata {
            name: world.name.clone(),
            version: release.version.clone(),
            url: release.url.clone(),
        }
    }
}

impl InstalledWorldMetadata {
    pub const FILENAME: &'static str = "apwm_metadata.toml";

    pub fn load(path: impl AsRef<Path>) -> Result<Option<Self>> {
        let data = match fs::read_to_string(path) {
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
            data => data.context("failed to read installed world metadata")?,
        };

        let data = toml::from_str::<Self>(&data)?;

        Ok(Some(data))
    }

    pub fn load_from_world_dir(world_dir_path: impl AsRef<Path>) -> Result<Option<Self>> {
        let path = world_dir_path.as_ref().join(Self::FILENAME);

        Self::load(path)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let data = toml::to_string_pretty(self)?;

        fs::write(path, data)?;

        Ok(())
    }

    pub fn save_to_world_dir(&self, world_dir_path: impl AsRef<Path>) -> Result<()> {
        let path = world_dir_path.as_ref().join(Self::FILENAME);

        self.save(path)
    }
}
