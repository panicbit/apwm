use anyhow::Result;
use apwm::cli::Cli;
use clap::Parser;

fn main() -> Result<()> {
    Cli::parse().run()?;

    Ok(())
}
