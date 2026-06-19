use clap::{Parser, Subcommand};

/// RCL - The Rust Command Line
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {

    //let cli_args = std::env::args();

    let args = Cli::parse();
    println!("Parsed args: {args:?}");

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}