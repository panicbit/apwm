use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::index::Index;

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

    pub fn apply_to_worlds_dir(&self, index: &Index, root: &Path) -> Result<()> {
        for (world_id, version) in &self.worlds {
            let world = index
                .find_world(world_id)
                .with_context(|| format!("failed to find world `{world_id}` in index"))?;

            world.find_release_by_version(version).with_context(|| {
                format!("failed to find version `{version}` of world `{world_id}`")
            })?;
        }

        for (world_id, version) in &self.worlds {
            crate::install(index, root, world_id, version)?;
        }

        Ok(())
    }
}
