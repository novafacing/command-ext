use command_ext::{CommandWrap, HasCommand};
use std::{ffi::OsStr, process::Command};
use typed_builder::TypedBuilder;

trait FriendlyCommandExt {
    fn message(&mut self, message: impl Into<String>) -> FriendlyCommand;
}

#[derive(TypedBuilder)]
struct FriendlyCommand<'a> {
    command: &'a mut Command,
    message: String,
}

impl<'a> HasCommand for FriendlyCommand<'a> {
    fn command(&self) -> &Command {
        self.command
    }

    fn command_mut(&mut self) -> &mut Command {
        self.command
    }
}

impl<'a> CommandWrap for FriendlyCommand<'a> {
    fn on_output(&mut self) {
        println!(
            "{} ({} {})",
            self.message,
            self.command().get_program().to_string_lossy(),
            self.command()
                .get_args()
                .collect::<Vec<_>>()
                .join(OsStr::new(" "))
                .to_string_lossy()
        )
    }
}

impl FriendlyCommandExt for Command {
    fn message(&mut self, message: impl Into<String>) -> FriendlyCommand {
        FriendlyCommand {
            command: self,
            message: message.into(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    Command::new("echo")
        .arg("x")
        .message("Running a happy little command")
        .output()?;
    Ok(())
}
