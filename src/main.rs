use std::process::Command;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Trolley {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        project_name: String,
    },
    Build,
    Generate,
    Deploy,
    Test,
    #[command(name = "--help")]
    Help,
}

fn main() -> Result<()> {
    let cli = Trolley::try_parse()?;

    Ok(())
}
