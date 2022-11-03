use crate::engine::Engine;
use clap::{Parser, Subcommand};
use console::{Term, TermFeatures};
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

use crate::level::sandbox;
use crate::program::Program;

mod engine;
mod level;
mod program;

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
        Command::Run => run(),
    }
}

fn level() {}

fn run() {
    let level = sandbox();
    let program = Program::light_to_the_right();
    let mut engine = Engine::new(&level, program);
    let term = Term::stdout();
    render(&engine, &term).unwrap();
    term.move_cursor_up(1).unwrap();
    while !engine.is_terminated() {
        engine.step();
        term.clear_line().unwrap();
        render(&engine, &term).unwrap();
        term.move_cursor_up(1).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

fn render(engine: &Engine, term: &Term) -> io::Result<()> {
    let (_height, width) = term.size();
    let from = engine.get_current_position() - width as i64 / 2;
    let to = from + width as i64;
    let tape = engine.render_tape(from, to);
    term.write_line(&tape)?;
    Ok(())
}
