use crate::terminal::commands::common::*;

pub fn execute_echo(app_state: &mut AppState, text: String) -> Result<(), CommandError> {
    println!("{}", text);
    Ok(())
}