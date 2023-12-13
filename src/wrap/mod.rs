use std::{
    ffi::OsStr,
    path::Path,
    process::{Child, Command, CommandArgs, CommandEnvs, ExitStatus, Output, Stdio},
};

pub trait HasCommand {
    fn command(&self) -> &Command;
    fn command_mut(&mut self) -> &mut Command;
}

pub trait CommandWrap: HasCommand {
    #[allow(unused)]
    #[inline(always)]
    /// Called when an arg is added using [`arg`]
    fn on_arg<S: AsRef<OsStr>>(&mut self, arg: S) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when args are added using [`args`]
    fn on_args<I, S>(&mut self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
    }

    #[allow(unused)]
    #[inline(always)]
    /// Called when environment variables are configured using [`env`]
    fn on_env<K, V>(&mut self, key: K, val: V)
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
    }

    #[allow(unused)]
    #[inline(always)]
    /// Called when environment variables are configured using [`envs`]
    fn on_envs<'a, I, K, V>(&mut self, vars: I)
    where
        I: IntoIterator<Item = &'a (K, V)>,
        K: AsRef<OsStr> + 'a,
        V: AsRef<OsStr> + 'a,
    {
    }

    #[allow(unused)]
    #[inline(always)]
    /// Called when environment variables are removed using [`env_remove`]
    fn on_env_remove<K: AsRef<OsStr>>(&mut self, key: K) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when environment variables are cleared using [`env_clear`]
    fn on_env_clear(&mut self) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when the current directory is set using [`current_dir`]
    fn on_current_dir<P: AsRef<Path>>(&mut self, dir: P) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when stdin is set using [`stdin`]
    fn on_stdin(&mut self, cfg: &Stdio) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when stdout is set using [`stdout`]
    fn on_stdout(&mut self, cfg: &Stdio) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when stderr is set using [`stderr`]
    fn on_stderr(&mut self, cfg: &Stdio) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when the child process is spawned using [`spawn`]
    fn on_spawn(&mut self) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when output is created using [`output`]
    fn on_output(&mut self) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when status is obtained using [`status`]
    fn on_status(&mut self) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when the child process is spawned using [`spawn`]
    fn after_spawn(&mut self, child: &std::io::Result<Child>) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when output is created using [`output`]
    fn after_output(&mut self, output: &std::io::Result<Output>) {}

    #[allow(unused)]
    #[inline(always)]
    /// Called when status is obtained using [`status`]
    fn after_status(&mut self, status: &std::io::Result<ExitStatus>) {}

    /// Adds an argument to pass to the program.
    ///
    /// Only one argument can be passed per use. So instead of:
    ///
    /// ```no_run
    /// # std::process::Command::new("sh")
    /// .arg("-C /path/to/repo")
    /// # ;
    /// ```
    ///
    /// usage would be:
    ///
    /// ```no_run
    /// # std::process::Command::new("sh")
    /// .arg("-C")
    /// .arg("/path/to/repo")
    /// # ;
    /// ```
    ///
    /// To pass multiple arguments see [`args`].
    ///
    /// [`args`]: Command::args
    ///
    /// Note that the argument is not passed through a shell, but given
    /// literally to the program. This means that shell syntax like quotes,
    /// escaped characters, word splitting, glob patterns, substitution, etc.
    /// have no effect.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::Command;
    ///
    /// Command::new("ls")
    ///     .arg("-l")
    ///     .arg("-a")
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.on_arg(&arg);
        self.command_mut().arg(arg);
        self
    }

    /// Adds multiple arguments to pass to the program.
    ///
    /// To pass a single argument see [`arg`].
    ///
    /// [`arg`]: Command::arg
    ///
    /// Note that the arguments are not passed through a shell, but given
    /// literally to the program. This means that shell syntax like quotes,
    /// escaped characters, word splitting, glob patterns, substitution, etc.
    /// have no effect.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::Command;
    ///
    /// Command::new("ls")
    ///     .args(["-l", "-a"])
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.command_mut().args(args);
        self
    }

    /// Inserts or updates an explicit environment variable mapping.
    ///
    /// This method allows you to add an environment variable mapping to the spawned process or
    /// overwrite a previously set value. You can use [`Command::envs`] to set multiple environment
    /// variables simultaneously.
    ///
    /// Child processes will inherit environment variables from their parent process by default.
    /// Environment variables explicitly set using [`Command::env`] take precedence over inherited
    /// variables. You can disable environment variable inheritance entirely using
    /// [`Command::env_clear`] or for a single key using [`Command::env_remove`].
    ///
    /// Note that environment variable names are case-insensitive (but
    /// case-preserving) on Windows and case-sensitive on all other platforms.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::Command;
    ///
    /// Command::new("ls")
    ///     .env("PATH", "/bin")
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.on_env(&key, &val);
        self.command_mut().env(key, val);
        self
    }

    /// Inserts or updates multiple explicit environment variable mappings.
    ///
    /// This method allows you to add multiple environment variable mappings to the spawned process
    /// or overwrite previously set values. You can use [`Command::env`] to set a single environment
    /// variable.
    ///
    /// Child processes will inherit environment variables from their parent process by default.
    /// Environment variables explicitly set using [`Command::envs`] take precedence over inherited
    /// variables. You can disable environment variable inheritance entirely using
    /// [`Command::env_clear`] or for a single key using [`Command::env_remove`].
    ///
    /// Note that environment variable names are case-insensitive (but case-preserving) on Windows
    /// and case-sensitive on all other platforms.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::{Command, Stdio};
    /// use std::env;
    /// use std::collections::HashMap;
    ///
    /// let filtered_env : HashMap<String, String> =
    ///     env::vars().filter(|&(ref k, _)|
    ///         k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH"
    ///     ).collect();
    ///
    /// Command::new("printenv")
    ///     .stdin(Stdio::null())
    ///     .stdout(Stdio::inherit())
    ///     .env_clear()
    ///     .envs(&filtered_env)
    ///     .spawn()
    ///     .expect("printenv failed to start");
    /// ```
    fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        let vars = vars.into_iter().collect::<Vec<_>>();
        self.on_envs(&vars);
        self.command_mut().envs(vars);
        self
    }

    /// Removes an explicitly set environment variable and prevents inheriting it from a parent
    /// process.
    ///
    /// This method will remove the explicit value of an environment variable set via
    /// [`Command::env`] or [`Command::envs`]. In addition, it will prevent the spawned child
    /// process from inheriting that environment variable from its parent process.
    ///
    /// After calling [`Command::env_remove`], the value associated with its key from
    /// [`Command::get_envs`] will be [`None`].
    ///
    /// To clear all explicitly set environment variables and disable all environment variable
    /// inheritance, you can use [`Command::env_clear`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::Command;
    ///
    /// Command::new("ls")
    ///     .env_remove("PATH")
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Self {
        self.on_env_remove(&key);
        self.command_mut().env_remove(key);
        self
    }

    /// Clears all explicitly set environment variables and prevents inheriting any parent process
    /// environment variables.
    ///
    /// This method will remove all explicitly added environment variables set via [`Command::env`]
    /// or [`Command::envs`]. In addition, it will prevent the spawned child process from inheriting
    /// any environment variable from its parent process.
    ///
    /// After calling [`Command::env_clear`], the iterator from [`Command::get_envs`] will be
    /// empty.
    ///
    /// You can use [`Command::env_remove`] to clear a single mapping.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::Command;
    ///
    /// Command::new("ls")
    ///     .env_clear()
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    fn env_clear(&mut self) -> &mut Self {
        self.on_env_clear();
        self.command_mut().env_clear();
        self
    }

    /// Sets the working directory for the child process.
    ///
    /// # Platform-specific behavior
    ///
    /// If the program path is relative (e.g., `"./script.sh"`), it's ambiguous
    /// whether it should be interpreted relative to the parent's working
    /// directory or relative to `current_dir`. The behavior in this case is
    /// platform specific and unstable, and it's recommended to use
    /// [`canonicalize`] to get an absolute program path instead.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::Command;
    ///
    /// Command::new("ls")
    ///     .current_dir("/bin")
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    ///
    /// [`canonicalize`]: crate::fs::canonicalize
    fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.on_current_dir(&dir);
        self.command_mut().current_dir(dir);
        self
    }

    /// Configuration for the child process's standard input (stdin) handle.
    ///
    /// Defaults to [`inherit`] when used with [`spawn`] or [`status`], and
    /// defaults to [`piped`] when used with [`output`].
    ///
    /// [`inherit`]: Stdio::inherit
    /// [`piped`]: Stdio::piped
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::{Command, Stdio};
    ///
    /// Command::new("ls")
    ///     .stdin(Stdio::null())
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        let cfg: Stdio = cfg.into();
        self.on_stdin(&cfg);
        self.command_mut().stdin(cfg);
        self
    }

    /// Configuration for the child process's standard output (stdout) handle.
    ///
    /// Defaults to [`inherit`] when used with [`spawn`] or [`status`], and
    /// defaults to [`piped`] when used with [`output`].
    ///
    /// [`inherit`]: Stdio::inherit
    /// [`piped`]: Stdio::piped
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::{Command, Stdio};
    ///
    /// Command::new("ls")
    ///     .stdout(Stdio::null())
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        let cfg: Stdio = cfg.into();
        self.on_stdout(&cfg);
        self.command_mut().stdout(cfg);
        self
    }

    /// Configuration for the child process's standard error (stderr) handle.
    ///
    /// Defaults to [`inherit`] when used with [`spawn`] or [`status`], and
    /// defaults to [`piped`] when used with [`output`].
    ///
    /// [`inherit`]: Stdio::inherit
    /// [`piped`]: Stdio::piped
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::{Command, Stdio};
    ///
    /// Command::new("ls")
    ///     .stderr(Stdio::null())
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        let cfg: Stdio = cfg.into();
        self.on_stderr(&cfg);
        self.command_mut().stderr(cfg);
        self
    }

    /// Executes the command as a child process, returning a handle to it.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::process::Command;
    ///
    /// Command::new("ls")
    ///     .spawn()
    ///     .expect("ls command failed to start");
    /// ```
    fn spawn(&mut self) -> std::io::Result<Child> {
        self.on_spawn();
        let child = self.command_mut().spawn();
        self.after_spawn(&child);
        child
    }

    /// Executes the command as a child process, waiting for it to finish and
    /// collecting all of its output.
    ///
    /// By default, stdout and stderr are captured (and used to provide the
    /// resulting output). Stdin is not inherited from the parent and any
    /// attempt by the child process to read from the stdin stream will result
    /// in the stream immediately closing.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::process::Command;
    /// use std::io::{self, Write};
    /// let output = Command::new("/bin/cat")
    ///     .arg("file.txt")
    ///     .output()
    ///     .expect("failed to execute process");
    ///
    /// println!("status: {}", output.status);
    /// io::stdout().write_all(&output.stdout).unwrap();
    /// io::stderr().write_all(&output.stderr).unwrap();
    ///
    /// assert!(output.status.success());
    /// ```
    fn output(&mut self) -> std::io::Result<Output> {
        self.on_output();
        let output = self.command_mut().output();
        self.after_output(&output);
        output
    }

    /// Executes a command as a child process, waiting for it to finish and
    /// collecting its status.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::process::Command;
    ///
    /// let status = Command::new("/bin/cat")
    ///     .arg("file.txt")
    ///     .status()
    ///     .expect("failed to execute process");
    ///
    /// println!("process finished with: {status}");
    ///
    /// assert!(status.success());
    /// ```
    fn status(&mut self) -> std::io::Result<ExitStatus> {
        self.on_status();
        let status = self.command_mut().status();
        self.after_status(&status);
        status
    }

    /// Returns the path to the program that was given to [`Command::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::process::Command;
    ///
    /// let cmd = Command::new("echo");
    /// assert_eq!(cmd.get_program(), "echo");
    /// ```
    fn get_program(&self) -> &OsStr {
        self.command().get_program()
    }

    /// Returns an iterator of the arguments that will be passed to the program.
    ///
    /// This does not include the path to the program as the first argument;
    /// it only includes the arguments specified with [`Command::arg`] and
    /// [`Command::args`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::new("echo");
    /// cmd.arg("first").arg("second");
    /// let args: Vec<&OsStr> = cmd.get_args().collect();
    /// assert_eq!(args, &["first", "second"]);
    /// ```
    fn get_args(&self) -> CommandArgs<'_> {
        self.command().get_args()
    }

    /// Returns an iterator of the environment variables explicitly set for the child process.
    ///
    /// Environment variables explicitly set using [`Command::env`], [`Command::envs`], and
    /// [`Command::env_remove`] can be retrieved with this method.
    ///
    /// Note that this output does not include environment variables inherited from the parent
    /// process.
    ///
    /// Each element is a tuple key/value pair `(&OsStr, Option<&OsStr>)`. A [`None`] value
    /// indicates its key was explicitly removed via [`Command::env_remove`]. The associated key for
    /// the [`None`] value will no longer inherit from its parent process.
    ///
    /// An empty iterator can indicate that no explicit mappings were added or that
    /// [`Command::env_clear`] was called. After calling [`Command::env_clear`], the child process
    /// will not inherit any environment variables from its parent process.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// cmd.env("TERM", "dumb").env_remove("TZ");
    /// let envs: Vec<(&OsStr, Option<&OsStr>)> = cmd.get_envs().collect();
    /// assert_eq!(envs, &[
    ///     (OsStr::new("TERM"), Some(OsStr::new("dumb"))),
    ///     (OsStr::new("TZ"), None)
    /// ]);
    /// ```
    fn get_envs(&self) -> CommandEnvs<'_> {
        self.command().get_envs()
    }

    /// Returns the working directory for the child process.
    ///
    /// This returns [`None`] if the working directory will not be changed.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// assert_eq!(cmd.get_current_dir(), None);
    /// cmd.current_dir("/bin");
    /// assert_eq!(cmd.get_current_dir(), Some(Path::new("/bin")));
    /// ```
    fn get_current_dir(&self) -> Option<&Path> {
        self.command().get_current_dir()
    }
}
