use crate::terminal::commands::common::*;

pub fn execute_echo(display: &mut Display, text: String) -> Result<(), CommandError> {
    display.output.push(text);
    Ok(())
}