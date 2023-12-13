//! Extension trait to log properties of a command
//!
//! # Example
//!
//! ```rust
//! # use std::process::Command;
//! # use command_ext::{CommandExtLog, CommandWrap};
//! # use log::Level;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let output = Command::new("echo")
//!     .arg("x")
//!     .log_args(Level::Debug)
//!     .log_status(Level::Info)
//!     .log_stdout(Level::Trace)
//!     .log_stderr(Level::Warn)
//!     .output()?;
//! # Ok(())
//! # }
//! ```

use log::{log, Level};
use std::{ffi::OsStr, process::Command};
use typed_builder::TypedBuilder;

use crate::{wrap::HasCommand, CommandWrap};
#[cfg(feature = "check")]
use crate::{CommandExtCheck, CommandExtError};

#[derive(TypedBuilder, Debug)]
pub struct CommandLog<'a> {
    command: &'a mut Command,
    #[builder(default, setter(into, strip_option))]
    /// The log level for args before execution
    args: Option<Level>,
    #[builder(default, setter(into, strip_option))]
    /// Whether to log the environment on execution
    envs: Option<Level>,
    #[builder(default, setter(into, strip_option))]
    /// Whether to log the current directory on execution
    current_dir: Option<Level>,
    #[builder(default, setter(into, strip_option))]
    /// Whether to log the status after execution
    status: Option<Level>,
    #[builder(default, setter(into, strip_option))]
    /// Whether to log stdout after execution
    stdout: Option<Level>,
    #[builder(default, setter(into, strip_option))]
    /// Whether to log stderr after execution
    stderr: Option<Level>,
}

impl<'a> CommandLog<'a> {
    fn log_before(&mut self) {
        if let Some(args) = self.args {
            log!(
                args,
                "args: {} {}",
                self.command().get_program().to_string_lossy(),
                self.command()
                    .get_args()
                    .collect::<Vec<_>>()
                    .join(OsStr::new(" "))
                    .to_string_lossy()
            );
        }

        if let Some(envs) = self.envs {
            self.command().get_envs().for_each(|(k, v)| {
                log!(
                    envs,
                    "envs: {}={}",
                    k.to_string_lossy(),
                    v.unwrap_or_default().to_string_lossy()
                );
            });
        }

        if let Some(current_dir) = self.current_dir {
            log!(
                current_dir,
                "current_dir: {}",
                self.command()
                    .get_current_dir()
                    .map(|d| d.to_string_lossy())
                    .unwrap_or_default()
            );
        }
    }
}

impl<'a> HasCommand for CommandLog<'a> {
    fn command(&self) -> &Command {
        self.command
    }

    fn command_mut(&mut self) -> &mut Command {
        self.command
    }
}

impl<'a> CommandWrap for CommandLog<'a> {
    fn on_spawn(&mut self) {
        self.log_before();
    }

    fn on_output(&mut self) {
        self.log_before();
    }

    fn on_status(&mut self) {
        self.log_before();
    }

    fn after_output(&mut self, output: &std::io::Result<std::process::Output>) {
        if let Ok(output) = output {
            if let Some(status) = self.status {
                log!(status, "status: {}", output.status);
            }
            if let Some(stdout) = self.stdout {
                let out = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !out.is_empty() {
                    log!(stdout, "stdout: {out}",);
                }
            }
            if let Some(stderr) = self.stderr {
                let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
                if !err.is_empty() {
                    log!(stderr, "stderr: {err}",);
                }
            }
        }
    }

    fn after_status(&mut self, status: &std::io::Result<std::process::ExitStatus>) {
        if let Ok(status) = status {
            if let Some(status_filter) = self.status {
                log!(status_filter, "status: {}", status);
            }
        }
    }
}

impl<'a> From<&'a mut Command> for CommandLog<'a> {
    fn from(value: &'a mut Command) -> Self {
        Self::builder().command(value).build()
    }
}

pub trait CommandExtLog {
    fn log_args<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>;
    fn log_envs<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>;
    fn log_current_dir<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>;
    fn log_status<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>;
    fn log_stdout<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>;
    fn log_stderr<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>;
}

impl CommandExtLog for Command {
    fn log_args<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>,
    {
        CommandLog::builder().command(self).args(filter).build()
    }

    fn log_envs<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>,
    {
        CommandLog::builder().command(self).envs(filter).build()
    }

    fn log_current_dir<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>,
    {
        CommandLog::builder()
            .command(self)
            .current_dir(filter)
            .build()
    }

    fn log_status<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>,
    {
        CommandLog::builder().command(self).status(filter).build()
    }

    fn log_stdout<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>,
    {
        CommandLog::builder().command(self).stdout(filter).build()
    }

    fn log_stderr<L>(&mut self, filter: L) -> CommandLog
    where
        L: Into<Level>,
    {
        CommandLog::builder().command(self).stderr(filter).build()
    }
}

impl<'a> CommandLog<'a> {
    pub fn log_args<L>(&'a mut self, filter: L) -> &'a mut CommandLog
    where
        L: Into<Level>,
    {
        self.args = Some(filter.into());
        self
    }

    pub fn log_envs<L>(&'a mut self, filter: L) -> &'a mut CommandLog
    where
        L: Into<Level>,
    {
        self.envs = Some(filter.into());
        self
    }

    pub fn log_current_dir<L>(&'a mut self, filter: L) -> &'a mut CommandLog
    where
        L: Into<Level>,
    {
        self.current_dir = Some(filter.into());
        self
    }

    pub fn log_status<L>(&'a mut self, filter: L) -> &'a mut CommandLog
    where
        L: Into<Level>,
    {
        self.status = Some(filter.into());
        self
    }

    pub fn log_stdout<L>(&'a mut self, filter: L) -> &'a mut CommandLog
    where
        L: Into<Level>,
    {
        self.stdout = Some(filter.into());
        self
    }

    pub fn log_stderr<L>(&'a mut self, filter: L) -> &'a mut CommandLog
    where
        L: Into<Level>,
    {
        self.stderr = Some(filter.into());
        self
    }
}

#[cfg(feature = "check")]
impl<'a> CommandExtCheck for CommandLog<'a> {
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
    use log::Level;
    use std::process::Command;
    use test_log::test;

    use crate::{CommandExtLog, CommandWrap};

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_args() -> anyhow::Result<()> {
        Command::new("echo")
            .arg("x")
            .log_args(Level::Error)
            .output()?;
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_envs() -> anyhow::Result<()> {
        Command::new("echo")
            .env("x", "y")
            .log_envs(Level::Error)
            .output()?;
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_current_dir() -> anyhow::Result<()> {
        Command::new("echo")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .log_current_dir(Level::Error)
            .output()?;
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_status() -> anyhow::Result<()> {
        Command::new("echo")
            .arg("x")
            .log_status(Level::Error)
            .output()?;

        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_stdout() -> anyhow::Result<()> {
        Command::new("echo")
            .arg("x")
            .log_stdout(Level::Error)
            .output()?;

        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_stderr() -> anyhow::Result<()> {
        Command::new("bash")
            .args(["-c", "echo y 1>&2"])
            .log_stderr(Level::Error)
            .output()?;

        Ok(())
    }
}
