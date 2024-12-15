use crate::terminal::commands::common::*;

pub fn execute_touch(display: &mut Display, command: Command) -> Result<(), CommandError> {
    if let Command::Touch { timestamp, targets } = command {
        for tgrt in targets{
            let mut new_trgt = PathBuf::from(&display.curr_dir);
            new_trgt.push(&tgrt);

            match File::create(&new_trgt) {
                Ok(_) => display.output.push(format!("Created file: {:?}", new_trgt)),
                Err(e) => return Ok(()),
            }
        }
    }
    Ok(())
}