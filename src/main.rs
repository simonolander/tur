use std::{fs, io::Result, process, thread};
use std::fs::create_dir_all;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand};
use console::Term;
use directories::ProjectDirs;

use crate::execution::TestCaseExecution;
use crate::level::{Level, sandbox};
use crate::level_dto::LevelDto;
use crate::levels::builtins;
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
    Program {
        #[command(subcommand)]
        command: ProgramCommand,
    },
    Run,
}

#[derive(Subcommand)]
enum LevelCommand {
    List,
    Create,
}

#[derive(Subcommand)]
enum ProgramCommand {
    Create {
        name: String
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Level { command } => level(command),
        Command::Program { command } => program(command),
        Command::Run => run(),
    }.unwrap();
}

fn program(command: ProgramCommand) -> Result<()> {
    match command {
        ProgramCommand::Create { name } => program_create(&name),
    }
}

fn program_create(name: &str) -> Result<()> {
    let term = Term::stdout();
    let file_path = program_dir()?.join(format!("{}.yaml", name));
    if file_path.exists() {
        term.write_line(&format!("Program {} already exists", name))?;
        return Ok(());
    }
    create_dir_all(file_path)?;
    process::Command::new("vim")
        .spawn()?
        .wait()?;
    Ok(())
}

fn level(command: LevelCommand) -> Result<()> {
    match command {
        LevelCommand::List => level_list(),
        LevelCommand::Create => level_create(),
    }
}

fn level_list() -> Result<()> {
    let term = Term::stdout();
    let level_dir = level_dir()?;
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
    for level in builtins() {
        term.write_line(&level.name)?;
    }
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
    let level_dir = level_dir()?;
    let mut result = process::Command::new("vim").spawn()?;
    result.wait()?;
    Ok(())
}

fn run() -> Result<()> {
    let level = sandbox();
    // let program = Program::light_the_world();
    let program = Program::light_left();
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

fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("org", "simonolander", "Tur").ok_or(Error::new(ErrorKind::NotFound, "Unable to find suitable project directory"))
}

fn level_dir() -> Result<PathBuf> {
    project_dirs().map(|dirs| dirs.data_dir().join("level"))
}

fn program_dir() -> Result<PathBuf> {
    project_dirs().map(|dirs| dirs.data_dir().join("program"))
}
