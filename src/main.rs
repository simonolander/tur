#![feature(file_create_new)]

use std::{fs, process, thread};
use std::cmp::max;
use std::fs::{create_dir_all, File, read_dir};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Error;
use anyhow::Result;
use clap::{Parser, Subcommand};
use console::Term;
use directories::ProjectDirs;
use prettytable::{row, Table};

use crate::execution::LevelExecution;
use crate::level::Level;
use crate::level_dto::LevelDto;
use crate::levels::builtins;
use crate::program::Program;
use crate::program_dto::ProgramDto;
use crate::render::{render, render_tce};

mod execution;
mod level;
mod level_dto;
mod levels;
mod program;
mod render;
mod program_dto;
mod programs;
mod outcome;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Operations related to levels
    Level {
        #[command(subcommand)]
        command: LevelCommand,
    },
    /// Program CRUD
    Program {
        #[command(subcommand)]
        command: ProgramCommand,
    },
    /// Run a program on a level
    Run {
        #[arg(short, long)]
        program: String,
        #[arg(short, long)]
        level: String,

        /// Amount of milliseconds to sleep between each step
        #[arg(short, long, default_value_t = 500)]
        sleep: u64,

        /// Test case to start with
        #[arg(short, long, default_value_t = 0)]
        test_case: usize,
    },
}

#[derive(Subcommand)]
enum LevelCommand {
    List,
}

#[derive(Subcommand)]
enum ProgramCommand {
    Create { name: String },
    List,
    Edit { name: String },
    Delete { name: String },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Level { command } => level(command),
        Command::Program { command } => program(command),
        Command::Run { program, level, sleep, test_case } => run(&program, &level, sleep, test_case),
    };

    if let Err(err) = result {
        Term::stdout().write_line(&err.to_string()).unwrap();
    }
}

fn program(command: ProgramCommand) -> Result<()> {
    match command {
        ProgramCommand::Create { name } => program_create(&name),
        ProgramCommand::Edit { name } => program_edit(&name),
        ProgramCommand::Delete { name } => program_delete(&name),
        ProgramCommand::List => program_list(),
    }
}

fn program_create(name: &str) -> Result<()> {
    let term = Term::stdout();
    let file_path = program_file(name)?;
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

fn program_edit(name: &str) -> Result<()> {
    let term = Term::stdout();
    let file_path = program_file(name)?;
    if !file_path.exists() {
        term.write_line(&format!("Program {} does not exist", name))?;
        return Ok(());
    }
    process::Command::new("vim")
        .arg(&file_path)
        .spawn()?
        .wait()?;
    Ok(())
}

fn program_delete(name: &str) -> Result<()> {
    let term = Term::stdout();
    let file_path = program_file(name)?;
    if !file_path.exists() {
        term.write_line(&format!("Program {} does not exist", name))?;
        return Ok(());
    }
    fs::remove_file(file_path)?;
    term.write_line(&format!("Program {} deleted", name))?;
    Ok(())
}

fn program_list() -> Result<()> {
    let dir = read_dir(program_dir()?)?;
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
    for program in programs::builtins() {
        table.add_row(row![program.name, "builtin"]);
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
    }
}

fn level_list() -> Result<()> {
    let level_dir = level_dir()?;
    let dir = read_dir(level_dir)?;
    let mut table = Table::new();
    table.set_titles(row!["Name", "Solved", "Type"]);
    for level in builtins() {
        table.add_row(row![level.name, false, "builtin"]);
    }
    for entry in dir {
        let file_contents = fs::read_to_string(entry?.path())?;
        let dto: LevelDto = serde_yaml::from_str(&file_contents)?;
        let level: Level = dto.into();
        table.add_row(row![level.name, false, "custom"]);
    }
    table.printstd();
    Ok(())
}

fn run(program_name: &str, level_name: &str, sleep: u64, test_case_index: usize) -> Result<()> {
    let sleep_duration = Duration::from_millis(max(sleep, 10));
    let mut level = find_level(level_name)
        .ok_or_else(|| Error::msg(format!("Level {} not found", level_name)))?;
    level.cases = level.cases[test_case_index..].to_vec();
    let program = find_program(program_name)?
        .ok_or_else(|| Error::msg(format!("Program {} not found", program_name)))?;
    let mut execution = LevelExecution::new(level, program);
    let term = Term::stdout();
    render(&term, &execution)?;
    while !execution.is_terminated() {
        thread::sleep(sleep_duration);
        execution.step();
        render(&term, &execution)?;
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
    for program in programs::builtins() {
        if program.name == name {
            return Ok(Some(program));
        }
    }

    let maybe_builtin = programs::builtins()
        .into_iter()
        .find(|program| program.name == name);
    Ok(maybe_builtin)
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

fn program_file(name: &str) -> Result<PathBuf> {
    let dir = program_dir()?;
    let path = dir.join(format!("{}.yaml", name));
    let parent = path.parent()
        .ok_or(Error::msg(format!("Could not get parent of path {}", path.to_string_lossy())))?;
    if parent == dir.as_path() {
        Ok(path)
    } else {
        Err(Error::msg(format!("Mismatched parents: expected {} but was {}", dir.to_string_lossy(), parent.to_string_lossy())))
    }
}