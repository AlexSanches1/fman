use clap::{Parser, Subcommand};
use crate::{copy_file_force, copy_file_safe, FmanError};

#[derive(Parser, Debug)]
#[command(name = "fman")]
#[command(author, version, about = "A simple file management CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Copy {
        src: String,
        dst: String,
        #[arg(short, long)]
        force: bool,
    },
    Move {
        src: String,
        dst: String,
    },
    Delete {
        target: String,
        #[arg(short, long)]
        force: bool,
    },
}

pub fn run() {
    if let Err(e) = try_run(std::env::args()) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

/// Runs the CLI logic with a given argument iterator (testable).
pub fn try_run<I, T>(args: I) -> Result<(), FmanError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Copy { src, dst, force } => {
            if force {
                copy_file_force(&src, &dst)
            } else {
                copy_file_safe(&src, &dst)
            }
        }
        Commands::Move { .. } => {
            eprintln!("Move not implemented.");
            Ok(())
        }
        Commands::Delete { .. } => {
            eprintln!("Delete not implemented.");
            Ok(())
        }
    }
}
