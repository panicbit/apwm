use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use semver::Version;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
    #[clap(long)]
    pub root: PathBuf,
}

#[derive(Subcommand)]
pub enum Command {
    List(List),
    Search,
    Install(Install),
}

#[derive(Args)]
pub struct List {
    #[clap(long, short, value_enum, default_value_t = Format::default())]
    pub format: Format,
}

#[derive(ValueEnum, Default, Clone)]
pub enum Format {
    #[default]
    Text,
    Json,
}

#[derive(Args)]
pub struct Install {
    pub world: String,
    #[clap(long, short)]
    /// world version to install, defaults to latest non-prerelease version
    pub version: Option<Version>,
}
