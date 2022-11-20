use std::io::Result;

use console::Term;

use crate::execution::TestCaseExecution;

const WINDOW_SIZE: i64 = 64;
const WINDOW_OFFSET: i64 = 32;

pub fn render(term: &Term, engine: &TestCaseExecution) -> Result<()> {
    let pos = engine.get_current_position();
    let from = from(pos);
    let to = from + WINDOW_SIZE + 1;

    // Render position
    term.clear_line()?;
    let position_line = " ".repeat((pos - from) as usize) + "v";
    term.write_line(&position_line)?;

    // Render tape
    let mut tape_line = String::new();
    for i in from..to {
        if engine.get_tape_at(i) {
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
    term.clear_line()?;
    term.write_line(&tick_line)?;
    let mut number_line = String::new();
    for n in 0..=WINDOW_SIZE / 16 {
        number_line += &format!("{:<16}", from + n * 16);
    }
    term.clear_line()?;
    term.write_line(&number_line)?;

    // Reset
    term.move_cursor_up(4)
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

