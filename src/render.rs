use std::io::Result;

use console::Term;

use crate::execution::TestCaseExecution;

pub fn render(term: &Term, engine: &TestCaseExecution) -> Result<()> {
    let pos = engine.get_current_position();
    let from = if pos < 0 {
        (pos / 16 * 16) - 32
    } else {
        (pos / 16 * 16) - 16
    };
    let to = from + 49;

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
    tick_line += &format!("{:<16}", '|');
    tick_line += &format!("{:<16}", '|');
    tick_line += &format!("{:<16}", '|');
    tick_line += &format!("{:<16}", '|');
    term.clear_line()?;
    term.write_line(&tick_line)?;
    let mut number_line = String::new();
    number_line += &format!("{:<16}", from);
    number_line += &format!("{:<16}", from + 16);
    number_line += &format!("{:<16}", from + 32);
    number_line += &format!("{:<16}", from + 48);
    term.clear_line()?;
    term.write_line(&number_line)?;

    // Reset
    term.move_cursor_up(4)
}
