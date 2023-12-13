use command_ext::CommandExtCheck;
use std::process::Command;
fn main() -> anyhow::Result<()> {
    Command::new("echo").arg("x").check()?; // Ok!
    Command::new("noexistcmd")
        .arg("badarg")
        .check()
        .map_err(|e| {
            // StdIoError(Os { code: 2, kind: NotFound, message: "No such file or directory" })
            eprintln!("{}", e);
        })
        .ok();
    Command::new("false")
        .check()
        .map_err(|e| {
            // Command failed with status (exit status: 1), stdout (), stderr ()
            eprintln!("{}", e);
        })
        .ok();
    Ok(())
}
