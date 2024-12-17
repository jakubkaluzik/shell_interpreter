pub use crate::terminal::state::AppState;
pub use crate::terminal::state::Display;
pub use crate::terminal::cmd_defs::{Command, CommandError};

pub use std::env;
pub use std::path::PathBuf;
pub use std::path::Path;
pub use std::fs;
pub use std::io;
pub use std::fs::File;
use std::io::Write;

pub fn output_redirect(output: Vec<String>, path: String) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    for line in output {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}