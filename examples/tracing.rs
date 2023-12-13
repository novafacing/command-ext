use command_ext::{CommandExtCheck, CommandExtTrace};
use std::io::stdout;
use std::process::Command;
use tracing::{metadata::LevelFilter, Level};
use tracing_subscriber::{fmt, prelude::*, registry, Layer};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    registry()
        .with(
            fmt::layer()
                .with_writer(stdout)
                .with_filter(LevelFilter::TRACE),
        )
        .try_init()?;
    Command::new("bash")
        .args(["-c", "echo err >&2; echo ok"])
        .trace_args(Level::DEBUG)
        .trace_status(Level::INFO)
        .trace_stdout(Level::TRACE)
        .trace_stderr(Level::WARN)
        .check()?;
    Ok(())
}
