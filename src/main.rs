use clap::{Parser, Subcommand};

mod cat;
mod ls;
mod pwd;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Output a file to stdout
    Cat,
    /// List files and/or directories
    Ls {
        /// Long listing
        #[arg(short = 'l')]
        long_output: bool,
    },
    /// Display the current directory
    Pwd,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Cat => {
            let cmd = cat::Cat {};

            cmd.process();
        },
        Commands::Ls { long_output } => {
            let cmd = ls::Ls {
                long_output: *long_output,
            };

            cmd.process();
        },
        Commands::Pwd => {
            let cmd = pwd::Pwd {};

            cmd.process();
        }
    }
}