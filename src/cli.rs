use std::io::stdout;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    Device {
        #[command(subcommand)]
        cmd: Device,
    },
    Completion {
        shell: Shell,
    },
}

#[derive(Debug, Subcommand, Clone)]
pub enum Device {
    List,
    Register,
    Monitor,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

impl Shell {
    pub fn generate(&self) {
        let shell = match self {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::Zsh => clap_complete::Shell::Zsh,
        };

        let mut stdout = stdout().lock();

        clap_complete::generate(
            shell,
            &mut Cli::command(),
            env!("CARGO_PKG_NAME"),
            &mut stdout,
        );
    }
}
