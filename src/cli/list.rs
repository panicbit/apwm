use anyhow::Result;

use crate::index::Index;

use super::{CommonArgs, Format};

#[derive(clap::Args)]
pub struct Args {
    #[clap(long, short, value_enum, default_value_t = Format::default())]
    pub format: Format,
}

pub fn run(_common: &CommonArgs, args: Args, index: &Index) -> Result<()> {
    match args.format {
        Format::Text => run_text(index),
        Format::Json => run_json(index),
    }
}

fn run_text(index: &Index) -> Result<()> {
    for (id, world) in &index.worlds {
        println!("{id} - {}", world.name);
    }

    Ok(())
}

fn run_json(index: &Index) -> Result<()> {
    let json = serde_json::to_string_pretty(index)?;

    println!("{json}");

    Ok(())
}
