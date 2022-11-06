use std::time::Duration;
use std::{io::Result, thread};

use clap::{Parser, Subcommand};
use console::Term;

use crate::engine::Engine;
use crate::level::sandbox;
use crate::program::Program;
use crate::render::render;

mod engine;
mod level;
mod level_dto;
mod program;
mod render;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Level,
    Run,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Level => level(),
        Command::Run => run().unwrap(),
    }
}

fn level() {
    let term = Term::stdout();
}

fn run() -> Result<()> {
    let level = sandbox();
    let program = Program::light_the_world();
    let mut engine = Engine::new(&level, program);
    let term = Term::stdout();
    render(&term, &engine)?;
    while !engine.is_terminated() {
        engine.step();
        render(&term, &engine)?;
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
