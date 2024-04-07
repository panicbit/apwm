use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

pub mod apply;
pub mod install;
pub mod list;
pub mod search;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
    #[clap(flatten)]
    pub common: CommonArgs,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let index = crate::index::load("index.toml")?;
        let common = &self.common;

        match self.command {
            Command::List(args) => list::run(common, args, &index),
            Command::Search(_args) => todo!(),
            Command::Install(args) => install::run(common, args, &index),
            Command::Apply(args) => apply::run(common, args, &index),
        }
    }
}

#[derive(clap::Args)]
pub struct CommonArgs {
    #[clap(long)]
    pub root: PathBuf,
}

#[derive(Subcommand)]
pub enum Command {
    List(list::Args),
    Search(search::Args),
    Install(install::Args),
    Apply(apply::Args),
}

#[derive(ValueEnum, Default, Clone)]
pub enum Format {
    #[default]
    Text,
    Json,
}
