use clap::{Parser, Subcommand};

mod ls;
mod cat;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List files and/or directories
    Ls {
        /// Long listing
        #[arg(short = 'l')]
        long_output: bool,
    },
    /// Display the current directory
    Pwd,
    /// Output a file to stdout
    Cat,
}

#[derive(Subcommand, Debug)]
enum AddCommands {

}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);

    let _x = 5;

    match &cli.command {
        Commands::Cat => {
            cat::process();
        },
        Commands::Ls { long_output } => {
            ls::process(long_output);
        },
        Commands::Pwd => {
            println!("PWD!");
        }
    }
}