use std::fmt::Debug;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, io::Result, process, thread};

use clap::{Parser, Subcommand};
use console::Term;
use directories::ProjectDirs;

use crate::execution::TestCaseExecution;
use crate::level::{sandbox, Level};
use crate::level_dto::LevelDto;
use crate::program::Program;
use crate::render::render;

mod execution;
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
    Create,
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
        LevelCommand::List => level_list(),
        LevelCommand::Create => level_create(),
    }
}

fn level_list() -> Result<()> {
    let term = Term::stdout();
    let level_dir = level_dir();
    if !level_dir.exists() {
        term.write_line(&format!(
            "Initiating levels directory at {}",
            &level_dir.to_str().unwrap()
        ))?;
        fs::create_dir_all(&level_dir)?
    }
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

fn level_create() -> Result<()> {
    let term = Term::stdout();
    let level_dir = level_dir();
    let mut result = process::Command::new("vim").spawn()?;
    result.wait()?;
    Ok(())
}

fn run() -> Result<()> {
    let level = sandbox();
    let program = Program::light_the_world();
    let mut engine = TestCaseExecution::new(level.cases[0].initial_tape.clone(), program);
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

fn level_dir() -> PathBuf {
    project_dirs().data_dir().join("level")
}
