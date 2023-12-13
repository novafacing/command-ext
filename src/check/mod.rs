//! Extension trait to check the output of a command

use crate::error::CommandExtError;
use std::process::{Command, Output};

/// Extension trait for [`std::process::Command`] to check the output of a command
pub trait CommandExtCheck {
    /// The error type for the result of checking for an error status
    type Error;

    /// Check the result of a command, returning an error containing the output and
    /// error stream content if the status is not success
    fn check(&mut self) -> Result<Output, Self::Error>;
}

impl CommandExtCheck for Command {
    type Error = CommandExtError;

    /// Check the result of a command, returning an error containing the status, output
    /// and error stream content if the status is not success
    fn check(&mut self) -> Result<Output, Self::Error> {
        self.output().map_err(CommandExtError::from).and_then(|r| {
            r.status
                .success()
                .then_some(r.clone())
                .ok_or_else(|| CommandExtError::Check {
                    status: r.status,
                    stdout: String::from_utf8_lossy(&r.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&r.stderr).to_string(),
                })
        })
    }
}

#[cfg(test)]
mod test {
    use std::process::Command;

    use crate::{CommandExtCheck, CommandExtError};

    #[test]
    #[cfg_attr(miri, ignore)]
    /// Check that a successful command returns a success output
    fn test_success() {
        let output = Command::new("echo").arg("x").check();
        match output {
            Ok(output) => assert_eq!(
                String::from_utf8_lossy(&output.stdout),
                "x\n",
                "Output mismatch"
            ),
            Err(e) => panic!("Unexpected error from command: {}", e),
        };
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    /// Test that a command that doesn't exist returns a wrapped IO error
    fn test_nocmd() {
        let output = Command::new("asdfasdfasdfasdfjkljkljkl").check();

        match output {
            Ok(output) => panic!("Unexpected success from command: {:?}", output),
            Err(e) => assert!(matches!(e, CommandExtError::StdIoError(_))),
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    /// Test that a command which fails by returning a nonzero status code returns a check error
    fn test_failure() {
        let output = Command::new("false").check();

        match output {
            Ok(output) => panic!("Unexpected success from command: {:?}", output),
            Err(e) => assert!(matches!(
                e,
                CommandExtError::Check {
                    status: _,
                    stdout: _,
                    stderr: _
                }
            )),
        }
    }
}
