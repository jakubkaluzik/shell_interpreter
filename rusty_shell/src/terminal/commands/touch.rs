use crate::terminal::commands::common::*;

pub fn execute_touch(curr_dir: &mut String, output: &mut Vec<String>, command: Command) -> Result<(), CommandError> {
    if let Command::Touch { timestamp, targets } = command {
        for tgrt in targets{
            let mut new_trgt = PathBuf::from(&curr_dir);
            new_trgt.push(&tgrt);

            match File::create(&new_trgt) {
                Ok(_) => output.push(format!("Created file: {:?}", new_trgt)),
                Err(e) => return Ok(()),
            }
        }
    }
    Ok(())
}