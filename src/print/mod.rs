//! Extension trait to log properties of a command
//!
//! # Example
//!
//! ```rust
//! # use std::process::Command;
//! # use command_ext::{CommandExtPrint, CommandWrap};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let output = Command::new("echo")
//!     .arg("x")
//!     .print_args()
//!     .print_status()
//!     .print_stdout()
//!     .print_stderr()
//!     .output()?;
//! # Ok(())
//! # }
//! ```

use std::{ffi::OsStr, process::Command};
use typed_builder::TypedBuilder;

use crate::{wrap::HasCommand, CommandWrap};
#[cfg(feature = "check")]
use crate::{CommandExtCheck, CommandExtError};

#[derive(TypedBuilder, Debug)]
pub struct CommandPrint<'a> {
    command: &'a mut Command,
    #[builder(default, setter(into))]
    /// The log level for args before execution
    args: bool,
    #[builder(default, setter(into))]
    /// Whether to log the environment on execution
    envs: bool,
    #[builder(default, setter(into))]
    /// Whether to log the current directory on execution
    current_dir: bool,
    #[builder(default, setter(into))]
    /// Whether to log the status after execution
    status: bool,
    #[builder(default, setter(into))]
    /// Whether to log stdout after execution
    stdout: bool,
    #[builder(default, setter(into))]
    /// Whether to log stderr after execution
    stderr: bool,
}

impl<'a> CommandPrint<'a> {
    fn print_before(&mut self) {
        if self.args {
            println!(
                "args: {} {}",
                self.command().get_program().to_string_lossy(),
                self.command()
                    .get_args()
                    .collect::<Vec<_>>()
                    .join(OsStr::new(" "))
                    .to_string_lossy()
            );
        }

        if self.envs {
            self.command().get_envs().for_each(|(k, v)| {
                println!(
                    "envs: {}={}",
                    k.to_string_lossy(),
                    v.unwrap_or_default().to_string_lossy()
                );
            });
        }

        if self.current_dir {
            println!(
                "current_dir: {}",
                self.command()
                    .get_current_dir()
                    .map(|d| d.to_string_lossy())
                    .unwrap_or_default()
            );
        }
    }
}

impl<'a> HasCommand for CommandPrint<'a> {
    fn command(&self) -> &Command {
        self.command
    }

    fn command_mut(&mut self) -> &mut Command {
        self.command
    }
}

impl<'a> CommandWrap for CommandPrint<'a> {
    fn on_spawn(&mut self) {
        self.print_before();
    }

    fn on_output(&mut self) {
        self.print_before();
    }

    fn on_status(&mut self) {
        self.print_before();
    }

    fn after_output(&mut self, output: &std::io::Result<std::process::Output>) {
        if let Ok(output) = output {
            if self.status {
                println!("status: {}", output.status);
            }
            if self.stdout {
                let out = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !out.is_empty() {
                    println!("stdout: {out}",);
                }
            }
            if self.stderr {
                let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
                if !err.is_empty() {
                    println!("stderr: {err}",);
                }
            }
        }
    }

    fn after_status(&mut self, status: &std::io::Result<std::process::ExitStatus>) {
        if let Ok(status) = status {
            if self.status {
                println!("status: {}", status);
            }
        }
    }
}

impl<'a> From<&'a mut Command> for CommandPrint<'a> {
    fn from(value: &'a mut Command) -> Self {
        Self::builder().command(value).build()
    }
}

pub trait CommandExtPrint {
    fn print_args(&mut self) -> CommandPrint;
    fn print_envs(&mut self) -> CommandPrint;
    fn print_current_dir(&mut self) -> CommandPrint;
    fn print_status(&mut self) -> CommandPrint;
    fn print_stdout(&mut self) -> CommandPrint;
    fn print_stderr(&mut self) -> CommandPrint;
}

impl CommandExtPrint for Command {
    fn print_args(&mut self) -> CommandPrint
    {
        CommandPrint::builder().command(self).args(true).build()
    }

    fn print_envs(&mut self) -> CommandPrint
    {
        CommandPrint::builder().command(self).envs(true).build()
    }

    fn print_current_dir(&mut self) -> CommandPrint
    {
        CommandPrint::builder()
            .command(self)
            .current_dir(true)
            .build()
    }

    fn print_status(&mut self) -> CommandPrint
    {
        CommandPrint::builder().command(self).status(true).build()
    }

    fn print_stdout(&mut self) -> CommandPrint
    {
        CommandPrint::builder().command(self).stdout(true).build()
    }

    fn print_stderr(&mut self) -> CommandPrint
    {
        CommandPrint::builder().command(self).stderr(true).build()
    }
}

impl<'a> CommandPrint<'a> {
    pub fn print_args(&'a mut self) -> &'a mut CommandPrint
    {
        self.args = true;
        self
    }

    pub fn print_envs(&'a mut self) -> &'a mut CommandPrint
    {
        self.envs = true;
        self
    }

    pub fn print_current_dir(&'a mut self) -> &'a mut CommandPrint
    {
        self.current_dir = true;
        self
    }

    pub fn print_status(&'a mut self) -> &'a mut CommandPrint
    {
        self.status = true;
        self
    }

    pub fn print_stdout(&'a mut self) -> &'a mut CommandPrint
    {
        self.stdout = true;
        self
    }

    pub fn print_stderr(&'a mut self) -> &'a mut CommandPrint
    {
        self.stderr = true;
        self
    }
}

#[cfg(feature = "check")]
impl<'a> CommandExtCheck for CommandPrint<'a> {
    type Error = CommandExtError;

    fn check(&mut self) -> Result<std::process::Output, Self::Error> {
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
    use test_log::test;

    use crate::{CommandExtPrint, CommandWrap};

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_args() -> anyhow::Result<()> {
        Command::new("echo")
            .arg("x")
            .print_args()
            .output()?;
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_envs() -> anyhow::Result<()> {
        Command::new("echo")
            .env("x", "y")
            .print_envs()
            .output()?;
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_current_dir() -> anyhow::Result<()> {
        Command::new("echo")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .print_current_dir()
            .output()?;
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_status() -> anyhow::Result<()> {
        Command::new("echo")
            .arg("x")
            .print_status()
            .output()?;

        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_stdout() -> anyhow::Result<()> {
        Command::new("echo")
            .arg("x")
            .print_stdout()
            .output()?;

        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_stderr() -> anyhow::Result<()> {
        Command::new("bash")
            .args(["-c", "echo y 1>&2"])
            .print_stderr()
            .output()?;

        Ok(())
    }
}
