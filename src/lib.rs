//! # CommandExt
//!
//! `CommandExt` is a set of convenient extension traits for `std::process::Command` which
//! make it easier to use, particularly in cargo scripts where many commands may be executed
//! with different requirements for error checking, logging, and so forth.
//!
//! ## CommandExtCheck
//!
//! `CommandExtCheck` allows you to check the result of a command and get a nicely packaged
//! error containing printable output and error streams.
//!
//! ```rust
//! use std::process::Command;
//! use command_ext::CommandExtCheck;
//! # fn main() -> anyhow::Result<()> {
//! Command::new("echo").arg("x").check()?; // Ok!
//! Command::new("noexistcmd").arg("badarg").check().map_err(|e| {
//!     // StdIoError(Os { code: 2, kind: NotFound, message: "No such file or directory" })
//!     eprintln!("{}", e);
//! }).ok();
//! Command::new("false").check().map_err(|e| {
//!     // Command failed with status (exit status: 1), stdout (), stderr ()
//!     eprintln!("{}", e);
//! }).ok();
//! # Ok(())
//! # }
//! ```
//!
//! Usually, scripts probably will just use `Command::new("cmd").args(["arg1", "arg2"]).check()?`.
//!
//! ## CommandExtLog
//!
//! `CommandExtLog` allows you to add customizable logging to your commands.
//!
//! ```rust
//! use std::process::Command;
//! use command_ext::{CommandExtCheck, CommandExtLog};
//! use env_logger::Builder;
//! use log::{LevelFilter, Level};
//! # fn main() -> anyhow::Result<()> {
//! Builder::new().filter_level(LevelFilter::max()).init();
//! Command::new("bash")
//!     .args(["-c", "echo err >&2; echo ok"])
//!     .log_args(Level::Debug)
//!     .log_status(Level::Info)
//!     .log_stdout(Level::Trace)
//!     .log_stderr(Level::Warn)
//!     .check()?;
//! # Ok(())
//! # }
//! ```
//!
//! This logs:
//!
//! ```txt
//! [2023-12-13T21:04:17Z DEBUG command_ext::log] args: bash -c echo err >&2; echo ok
//! [2023-12-13T21:04:17Z INFO  command_ext::log] status: exit status: 0
//! [2023-12-13T21:04:17Z TRACE command_ext::log] stdout: ok
//! [2023-12-13T21:04:17Z WARN  command_ext::log] stderr: err
//! ```
//!
//! ## CommandExtTrace
//!
//! `CommandExtTrace` works very similarly to `CommandExtLog`
//!
//! ```rust
//! use command_ext::{CommandExtCheck, CommandExtTrace};
//! use std::io::stdout;
//! use std::process::Command;
//! use tracing::{metadata::LevelFilter, Level};
//! use tracing_subscriber::{fmt, prelude::*, registry, Layer};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     registry()
//!         .with(
//!             fmt::layer()
//!                 .with_writer(stdout)
//!                 .with_filter(LevelFilter::TRACE),
//!         )
//!         .try_init()?;
//!     Command::new("bash")
//!         .args(["-c", "echo err >&2; echo ok"])
//!         .trace_args(Level::DEBUG)
//!         .trace_status(Level::INFO)
//!         .trace_stdout(Level::TRACE)
//!         .trace_stderr(Level::WARN)
//!         .check()?;
//! #     Ok(())
//! # }
//! ```
//!
//! This traces:
//!
//! ```txt
//! 2023-12-13T21:06:31.739932Z DEBUG command_ext::trace: args: bash -c echo err >&2; echo ok
//! 2023-12-13T21:06:31.741100Z  INFO command_ext::trace: status: exit status: 0
//! 2023-12-13T21:06:31.741138Z TRACE command_ext::trace: stdout: ok
//! 2023-12-13T21:06:31.741147Z  WARN command_ext::trace: stderr: err
//! ```
//!
//! ## CommandWrap
//!
//! For other cases where you might want to hook into what `Command` is doing, you can use
//! `CommandWrap` to implement your own wrappers. See the examples for more details.

pub mod error;
pub use error::CommandExtError;

pub mod wrap;
pub use wrap::{CommandWrap, HasCommand};

#[cfg(feature = "check")]
pub mod check;
#[cfg(feature = "check")]
pub use check::CommandExtCheck;

#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "log")]
pub use log::CommandExtLog;

#[cfg(feature = "tracing")]
pub mod trace;
#[cfg(feature = "tracing")]
pub use trace::CommandExtTrace;

#[cfg(all(feature = "check", feature = "log", feature = "tracing"))]
pub trait CommandExt: CommandExtCheck + CommandExtLog + CommandExtTrace {}

#[cfg(all(feature = "check", feature = "log", not(feature = "tracing")))]
pub trait CommandExt: CommandExtCheck + CommandExtLog {}

#[cfg(all(feature = "check", not(feature = "log"), feature = "tracing"))]
pub trait CommandExt: CommandExtCheck + CommandExtTrace {}

#[cfg(all(not(feature = "check"), feature = "log", feature = "tracing"))]
pub trait CommandExt: CommandExtLog + CommandExtTrace {}

#[cfg(all(feature = "check", not(feature = "log"), not(feature = "tracing")))]
pub trait CommandExt: CommandExtCheck {}

#[cfg(all(not(feature = "check"), feature = "log", not(feature = "tracing")))]
pub trait CommandExt: CommandExtLog {}

#[cfg(all(not(feature = "check"), not(feature = "log"), feature = "tracing"))]
pub trait CommandExt: CommandExtTrace {}
