use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Level,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Level => level(),
    }
}

fn level() {}
