use anyhow::{anyhow, Result};
use std::{
    io::prelude::*,
    process::{Command, Stdio},
};

use crate::error::EasyGHError;

pub struct Fzf(std::process::Child);

impl Fzf {
    pub fn new() -> Result<Self> {
        #[cfg(windows)]
        let program =
            which::which("fzf.exe").map_err(|_| anyhow!(EasyGHError::ERR_FZF_NOT_FOUND))?;
        #[cfg(not(windows))]
        let program = which::which("fzf").map_err(|_| anyhow!(EasyGHError::FzfNotFound))?;
        match Command::new(program)
            .arg("-m")
            .arg("--height=20")
            .arg("--layout=reverse-list")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => Err(anyhow!(EasyGHError::FzfSpawnError)),
            Ok(process) => Ok(Fzf(process)),
        }
    }

    pub fn write_to_stdin<T: std::fmt::Display>(&mut self, input: &[T]) -> Result<()> {
        let stdin = self.0.stdin.as_mut().unwrap();
        input.iter().for_each(|plugin| {
            writeln!(stdin, "{}", plugin.to_string()).unwrap();
        });
        Ok(())
    }

    pub fn read_from_stdout(&mut self) -> Result<String> {
        let mut s = String::new();
        if let Err(why) = self.0.stdout.as_mut().unwrap().read_to_string(&mut s) {
            Err(anyhow!(EasyGHError::FzfReadError))?;
        }
        Ok(s)
    }

    pub fn get_selected<T: std::fmt::Display + Clone>(
        &mut self,
        possible_plugins: &[T],
    ) -> Result<Vec<T>> {
        let selected_plugins = self
            .read_from_stdout()?
            .lines()
            .map(|line| {
                possible_plugins
                    .iter()
                    .find(|plugin| plugin.to_string() == *line)
                    .unwrap()
                    .clone()
            })
            .collect::<Vec<T>>();
        Ok(selected_plugins)
    }
}
