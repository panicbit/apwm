use std::path::PathBuf;

use anyhow::Result;

use crate::index::Index;
use crate::manifest::Manifest;

use super::CommonArgs;

#[derive(clap::Args)]
pub struct Args {
    manifest: PathBuf,
}

pub fn run(common: &CommonArgs, args: Args, index: &Index) -> Result<()> {
    let manifest = Manifest::load(args.manifest)?;

    manifest.apply_to_worlds_dir(index, &common.root)?;

    Ok(())
}
