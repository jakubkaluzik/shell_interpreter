use crate::terminal::commands::common::*;

pub fn execute_echo(display: &mut Display, command: Command) -> Result<(), CommandError> {
    if let Command::Echo { text } = command {
        let processed_text = text.replace("\\n", "\n");
        display.output.push(processed_text);
    }
    Ok(())
}