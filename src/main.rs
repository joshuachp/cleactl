use clap::Parser;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::Cli;

mod cli;
mod config;

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    match cli.cmd {
        cli::Command::Device { cmd } => match cmd {
            cli::Device::List => info!("list"),
            cli::Device::Monitor => info!("monitor"),
            cli::Device::Register => info!("register"),
        },
        cli::Command::Completion { shell } => shell.generate(),
    }

    Ok(())
}
