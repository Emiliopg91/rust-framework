#[cfg(test)]
mod tests;

pub mod errors;

use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    path::Path,
    sync::{Arc, LazyLock},
    time::Instant,
};

use tokio::{process::Command, sync::RwLock};

use crate::{
    logger::{Logger, provider::Provider},
    shell::errors::{Result, ShellError},
};

pub struct ShellOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
    pub elapsed: f64,
}

static LOGGER: LazyLock<Arc<Logger>> = LazyLock::new(|| Provider::get_logger("Shell"));
static COMMANDS: LazyLock<RwLock<HashMap<String, bool>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub struct Shell {
    command: Command,
    program: String,
    args: Vec<String>,
}

impl Shell {
    async fn run_command(shell: &mut Shell, raise_if_not_0: bool) -> Result<ShellOutput> {
        let mut cmd = shell.program.clone();
        if !shell.args.is_empty() {
            cmd = format!("{} {}", cmd, shell.args.join(" "))
        }

        Self::check_program_exists(&shell.program).await?;

        LOGGER.info(format!("Running command {}...", cmd));
        let t0 = Instant::now();

        let res = shell
            .command
            .output()
            .await
            .map_err(|e| ShellError::CommandFailed(cmd.clone(), e))?;

        let elapsed = t0.elapsed().as_secs_f64();

        let status = res
            .status
            .code()
            .ok_or_else(|| ShellError::TerminatedBySignal(cmd.clone()))?;

        LOGGER.info(format!(
            "Execution finished after {:.3} with status {}",
            elapsed, status
        ));

        if status != 0 && raise_if_not_0 {
            return Err(ShellError::NonZeroStatus(cmd, status));
        }

        Ok(ShellOutput {
            status,
            stderr: String::from_utf8_lossy(&res.stderr).to_string(),
            stdout: String::from_utf8_lossy(&res.stdout).to_string(),
            elapsed,
        })
    }

    pub async fn check_program_exists<T>(program: T) -> Result<()>
    where
        T: AsRef<OsStr>,
    {
        let program = program.as_ref().to_string_lossy().to_string();

        if program != "which" {
            let map = COMMANDS.read().await;
            if let Some(v) = map.get(&program) {
                if !v {
                    return Err(ShellError::CommandNotFound(program));
                } else {
                    return Ok(());
                }
            } else {
                drop(map);

                let success = Command::new("which")
                    .arg(&program)
                    .output()
                    .await
                    .unwrap()
                    .status
                    .success();

                let mut map = COMMANDS.write().await;
                map.insert(program.clone(), success);

                if !success {
                    return Err(ShellError::CommandNotFound(program.clone()));
                }
            }
        }

        Ok(())
    }

    pub fn command<T>(program: T) -> Result<Self>
    where
        T: AsRef<OsStr>,
    {
        let mut command = Command::new(&program);
        command.current_dir(env::current_dir().map_err(ShellError::InvalidCurrentDir)?);

        Ok(Self {
            command,
            program: program.as_ref().to_str().unwrap().to_string(),
            args: Vec::new(),
        })
    }

    pub fn cwd<T>(&mut self, working_dir: T) -> &mut Self
    where
        T: AsRef<Path>,
    {
        self.command.current_dir(working_dir);
        self
    }

    pub fn arg<T>(&mut self, arg: T) -> &mut Self
    where
        T: AsRef<OsStr>,
    {
        self.command.arg(&arg);
        self.args.push(arg.as_ref().to_str().unwrap().to_owned());
        self
    }

    pub fn args<T, I>(&mut self, args: T) -> &mut Self
    where
        T: IntoIterator<Item = I>,
        I: AsRef<OsStr>,
    {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    pub fn env<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.command.env(key, value);
        self
    }

    pub fn envs<I, K, V>(&mut self, entries: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        for (key, value) in entries {
            self.env(key, value);
        }
        self
    }

    pub async fn run(&mut self, err_if_status_non_0: bool) -> Result<ShellOutput> {
        Self::run_command(self, err_if_status_non_0).await
    }
}
