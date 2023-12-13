use std::process::ExitStatus;

use thiserror::Error;

#[derive(Error, Debug)]
/// An error when checking the result of a command
pub enum CommandExtError {
    #[error("Command failed with status ({status}), stdout ({stdout}), stderr ({stderr})")]
    Check {
        status: ExitStatus,
        stdout: String,
        stderr: String,
    },
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}
