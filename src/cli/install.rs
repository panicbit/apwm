use crate::index::Index;
use anyhow::Result;
use semver::Version;

use super::CommonArgs;

#[derive(clap::Args)]
pub struct Args {
    pub world: String,
    #[clap(long, short)]
    /// world version to install, defaults to latest non-prerelease version
    pub version: Option<Version>,
}

pub fn run(common: &CommonArgs, args: Args, index: &Index) -> Result<()> {
    crate::install(index, &common.root, &args.world, &args.version)
}
