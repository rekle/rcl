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

    //println!("{:?}", cli);

    match &cli.command {
        Commands::Cat => {
            cat::process();
        },
        Commands::Ls { long_output } => {
            ls::process(long_output);
        },
        Commands::Pwd => {
            pwd::process();
        }
    }
}