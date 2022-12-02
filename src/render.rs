use std::fmt::Debug;

use anyhow::Result;
use console::Term;

use crate::execution::{LevelExecution, TestCaseExecution, TestCaseExecutionState};

const WINDOW_SIZE: i64 = 64;
const WINDOW_OFFSET: i64 = 32;

pub fn render(term: &Term, le: &LevelExecution) -> Result<()> {
    term.clear_screen()?;
    term.write_line(&format!("Level: {}", le.level.name))?;
    term.write_line(&format!("Program: {}", le.program.name))?;
    term.write_line("Test cases:")?;
    for (index, tce) in le.executions.iter().enumerate() {
        match tce.get_state() {
            TestCaseExecutionState::Pending => {
                term.write_line(&format!("{:>4}: Pending", index))?;
            }
            TestCaseExecutionState::Running => {
                term.write_line(&format!("{:>4}: Running", index))?;
                term.write_line(&format!("      Steps: {}", tce.steps))?;
            }
            TestCaseExecutionState::Success => {
                term.write_line(&format!("{:>4}: Success", index))?;
                term.write_line(&format!("      Steps: {}", tce.steps))?;
            }
            TestCaseExecutionState::Failure { errors } => {
                term.write_line(&format!("{:>4}: Failure", index))?;
                for error in errors {
                    term.write_line(&format!("       - {}", error))?;
                }
            }
        }
    }
    if let Some(tce) = le.current_execution() {
        render_tce(term, tce)?;
    }
    if le.is_terminated() {
        term.write_line(&format!("Total steps: {}", le.get_steps()))?;
    }

    Ok(())
}

pub fn render_tce(term: &Term, tce: &TestCaseExecution) -> Result<()> {
    let pos = tce.get_current_position();
    let from = from(pos);
    let to = from + WINDOW_SIZE + 1;

    // Render position
    let position_line = " ".repeat((pos - from) as usize) + "v";
    term.write_line(&position_line)?;

    // Render tape
    let mut tape_line = String::new();
    for i in from..to {
        if tce.get_tape_at(i) {
            tape_line.push('■')
        } else {
            tape_line.push('□')
        }
    }
    term.write_line(&tape_line)?;

    // Render ticks
    let mut tick_line = String::new();
    for _ in 0..=WINDOW_SIZE / 16 {
        tick_line += &format!("{:<16}", '|');
    }
    term.write_line(&tick_line)?;
    let mut number_line = String::new();
    for n in 0..=WINDOW_SIZE / 16 {
        number_line += &format!("{:<16}", from + n * 16);
    }
    term.write_line(&number_line)?;
    Ok(())
}

fn from(pos: i64) -> i64 {
    let effective_position = pos + WINDOW_OFFSET;
    if effective_position < 0 && effective_position % WINDOW_SIZE != 0 {
        (effective_position / WINDOW_SIZE - 1) * WINDOW_SIZE - WINDOW_OFFSET
    } else {
        effective_position / WINDOW_SIZE * WINDOW_SIZE - WINDOW_OFFSET
    }
}

#[cfg(test)]
mod tests {
    use crate::render::{from, WINDOW_OFFSET, WINDOW_SIZE};

    #[test]
    fn test_from() {
        for n in -2..3 {
            let expected = -WINDOW_OFFSET + WINDOW_SIZE * n;
            for m in vec![0, -1, 1, -WINDOW_OFFSET, -WINDOW_OFFSET + WINDOW_SIZE - 1] {
                let pos = m + WINDOW_SIZE * n;
                let actual = from(pos);
                assert_eq!(actual, expected, "Expected from({}) to be {}, but was {}", pos, expected, actual);
            }
        }
    }
}

