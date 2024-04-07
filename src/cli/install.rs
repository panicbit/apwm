use std::fs::{self, File};
use std::io::{self, Cursor, Read, Seek};
use std::path::Path;

use crate::index::{Index, Release, World};
use crate::installed_world_metadata::InstalledWorldMetadata;
use anyhow::{Context, Result};
use semver::Version;
use zip::ZipArchive;

use super::CommonArgs;

#[derive(clap::Args)]
pub struct Args {
    pub world: String,
    #[clap(long, short)]
    /// world version to install, defaults to latest non-prerelease version
    pub version: Option<Version>,
}

pub fn run(common: &CommonArgs, args: Args, index: &Index) -> Result<()> {
    let root = &common.root;
    let world_id = &args.world;
    let world = index
        .find_world(world_id)
        .context("world not found in index, try updating it")?;

    let release = match args.version {
        None => world
            .find_latest_non_prerelease_release()
            .context("world has no (non-prerelease) releases")?,
        Some(version) => world
            .find_release_by_version(&version)
            .context("release not found")?,
    };

    eprintln!("Downloading {:?}", release.url);

    let mut world_data = Vec::new();

    reqwest::blocking::get(&release.url)?
        .error_for_status()?
        .read_to_end(&mut world_data)?;

    let world_data = Cursor::new(world_data);

    extract_world_into_worlds_dir(world_data, world_id, root, world, release)?;

    Ok(())
}

fn extract_world_into_worlds_dir<R: Read + Seek>(
    world_data: R,
    world_id: &str,
    worlds_dir: impl AsRef<Path>,
    world: &World,
    release: &Release,
) -> Result<()> {
    let mut world_data = ZipArchive::new(world_data)?;

    let prefix = format!("{}/", world_id);

    let world_path = worlds_dir.as_ref().join(world_id);

    let installed_world_metadata = InstalledWorldMetadata::load_from_world_dir(&world_path)?;
    let version = match installed_world_metadata {
        Some(meta) => format!("{}", meta.version),
        None => "unknown".to_string(),
    };

    if world_path.try_exists()? {
        eprintln!("Removing {} ({}) at {:?}", world_id, version, world_path);
        fs::remove_dir_all(&world_path)?;
    }

    for i in 0..world_data.len() {
        let mut entry = world_data.by_index(i)?;

        if !entry.name().starts_with(&prefix) {
            eprintln!("Ignoring {:?}", entry.name());
            continue;
        }

        let file_name = entry
            .enclosed_name()
            .context("world contains unsafe file name")?;

        let path = worlds_dir.as_ref().join(file_name);

        eprintln!("Creating {path:?}");

        if entry.is_dir() {
            fs::create_dir_all(path)?;
        } else {
            let mut file = File::create(path)?;

            io::copy(&mut entry, &mut file)?;
        }
    }

    InstalledWorldMetadata::from((world, release)).save_to_world_dir(world_path)?;

    Ok(())
}
