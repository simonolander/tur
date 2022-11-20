#![feature(file_create_new)]

use std::{fs, process, thread};
use std::fs::{create_dir_all, File, read_dir};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Error;
use anyhow::Result;
use clap::{Parser, Subcommand};
use console::Term;
use directories::ProjectDirs;
use prettytable::{format, row, Table};

use crate::execution::{LevelExecution, TestCaseExecution};
use crate::level::{Level, sandbox};
use crate::level_dto::LevelDto;
use crate::levels::builtins;
use crate::program::Program;
use crate::program_dto::ProgramDto;
use crate::render::render;

mod execution;
mod level;
mod level_dto;
mod levels;
mod program;
mod render;
mod program_dto;

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
    Run {
        #[arg(short, long)]
        program: String,
        #[arg(short, long)]
        level: String,
    },
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
    List,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Level { command } => level(command),
        Command::Program { command } => program(command),
        Command::Run { program, level } => run(&program, &level),
    };

    if let Err(err) = result {
        Term::stdout().write_line(&err.to_string()).unwrap();
    }
}

fn program(command: ProgramCommand) -> Result<()> {
    match command {
        ProgramCommand::Create { name } => program_create(&name),
        ProgramCommand::List => program_list(),
    }
}

fn program_create(name: &str) -> Result<()> {
    let term = Term::stdout();
    let file_path = program_dir()?.join(format!("{}.yaml", name));
    if file_path.exists() {
        term.write_line(&format!("Program {} already exists", name))?;
        return Ok(());
    }
    let mut file = File::create_new(&file_path)?;
    file.write_all(include_str!("template/program.yaml").replace("PROGRAM_NAME", name).as_bytes())?;
    process::Command::new("vim")
        .arg(&file_path)
        .spawn()?
        .wait()?;
    Ok(())
}

fn program_list() -> Result<()> {
    let dir = fs::read_dir(program_dir()?)?;
    let mut table = Table::new();
    table.set_titles(row!("Name", "Status"));
    for entry in dir {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().trim_end_matches(".yaml").to_string();
        let file_contents = fs::read_to_string(entry.path())?;
        let dto: ProgramDto = match serde_yaml::from_str(&file_contents) {
            Ok(dto) => dto,
            Err(err) => {
                table.add_row(row![file_name, err.to_string()]);
                continue;
            }
        };
        let program: Program = match dto.try_into() {
            Ok(program) => program,
            Err(err) => {
                table.add_row(row![file_name, err.to_string()]);
                continue;
            }
        };
        table.add_row(row![program.name, "ok"]);
    }
    if !table.is_empty() {
        table.printstd();
    } else {
        Term::stdout().write_line("No programs found. You can create programs by running:")?;
        Term::stdout().write_line("")?;
        Term::stdout().write_line("    tur program create <program name>")?;
        Term::stdout().write_line("")?;
    }
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
            &level_dir.to_string_lossy()
        ))?;
        create_dir_all(&level_dir)?
    }
    let dir = fs::read_dir(level_dir)?;
    term.write_line("Levels")?;
    term.write_line("------")?;
    for level in builtins() {
        term.write_line(&level.name)?;
    }
    for entry in dir {
        let file_contents = fs::read_to_string(entry?.path())?;
        let dto: LevelDto = serde_yaml::from_str(&file_contents)?;
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

fn run(program_name: &str, level_name: &str) -> Result<()> {
    let level = find_level(level_name)
        .ok_or_else(|| Error::msg(format!("Level {} not found", level_name)))?;
    let program = find_program(program_name)?
        .ok_or_else(|| Error::msg(format!("Program {} not found", program_name)))?;
    let mut execution = LevelExecution::new(level, program);
    let term = Term::stdout();
    render(&term, &execution.current_execution().unwrap())?;
    while !execution.is_terminated() {
        thread::sleep(Duration::from_millis(1000));
        execution.step();
        if let Some(tex) = execution.current_execution() {
            render(&term, tex)?;
        }
    }
    Ok(())
}

fn find_level(name: &str) -> Option<Level> {
    builtins().into_iter().find(|level| level.name == name)
}

fn find_program(name: &str) -> Result<Option<Program>> {
    for entry in read_dir(program_dir()?)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name != format!("{}.yaml", name) {
            continue;
        }
        let file_contents = fs::read_to_string(entry.path())?;
        let dto: ProgramDto = serde_yaml::from_str(&file_contents)?;
        let program = Program::try_from(dto)?;
        return Ok(Some(program));
    }

    Ok(None)
}

fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("org", "simonolander", "Tur").ok_or(Error::msg("Unable to find suitable project directory"))
}

fn level_dir() -> Result<PathBuf> {
    project_dirs().map(|dirs| dirs.data_dir().join("level"))
}

fn program_dir() -> Result<PathBuf> {
    let buf = project_dirs().map(|dirs| dirs.data_dir().join("program"))?;
    if !buf.exists() {
        create_dir_all(&buf)?;
    }
    Ok(buf)
}
