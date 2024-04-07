use std::collections::BTreeMap;

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Index {
    #[serde(default)]
    pub worlds: BTreeMap<String, World>,
}

impl Index {
    pub fn find_world(&self, world_id: &str) -> Option<&World> {
        self.worlds.get(world_id)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct World {
    pub name: String,
    #[serde(default)]
    pub releases: Vec<Release>,
}

impl World {
    pub fn find_latest_non_prerelease_release(&self) -> Option<&Release> {
        self.releases
            .iter()
            .filter(|release| release.version.pre.is_empty())
            .max_by_key(|release| &release.version)
    }

    pub fn find_release_by_version(&self, version: &Version) -> Option<&Release> {
        self.releases
            .iter()
            .find(|release| &release.version == version)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Release {
    pub url: String,
    pub version: Version,
}
