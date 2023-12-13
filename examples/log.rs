use command_ext::{CommandExtCheck, CommandExtLog};
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::process::Command;
fn main() -> anyhow::Result<()> {
    Builder::new().filter_level(LevelFilter::max()).init();
    Command::new("bash")
        .args(["-c", "echo err >&2; echo ok"])
        .log_args(Level::Debug)
        .log_status(Level::Info)
        .log_stdout(Level::Trace)
        .log_stderr(Level::Warn)
        .check()?;
    Ok(())
}
