use std::fmt::Debug;
use std::time::Duration;
use std::{fs, io::Result, thread};

use clap::{Parser, Subcommand};
use console::Term;
use directories::ProjectDirs;

use crate::engine::Engine;
use crate::level::{sandbox, Level};
use crate::level_dto::LevelDto;
use crate::program::Program;
use crate::render::render;

mod engine;
mod level;
mod level_dto;
mod levels;
mod program;
mod render;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Level {
        #[command(subcommand)]
        command: LevelCommand,
    },
    Run,
}

#[derive(Subcommand)]
enum LevelCommand {
    List,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Level { command } => level(command).unwrap(),
        Command::Run => run().unwrap(),
    }
}

fn level(command: LevelCommand) -> Result<()> {
    match command {
        List => level_list()?,
    }
    Ok(())
}

fn level_list() -> Result<()> {
    let term = Term::stdout();
    let level_dir = project_dirs().data_dir().join("level");
    let dir = fs::read_dir(level_dir)?;
    term.write_line("Levels")?;
    term.write_line("------")?;
    for entry in dir {
        let file_contents = fs::read_to_string(entry?.path())?;
        let dto: LevelDto = serde_yaml::from_str(&file_contents).unwrap();
        let level: Level = dto.into();
        term.write_line(&level.name)?;
    }
    Ok(())
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

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("org", "simonolander", "Tur").unwrap()
}
