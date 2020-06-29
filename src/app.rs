use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Output};

pub struct AppState {
    pub cli_path: PathBuf,
}

pub struct CliApp {
    app: Command,
}

impl CliApp {
    pub fn new(path: &PathBuf) -> CliApp {
        CliApp {
            app: Command::new(path),
        }
    }
    pub fn execute(
        &mut self,
        args: &Vec<String>,
        working_directory: &PathBuf,
        env_vars: &HashMap<String, String>,
    ) -> Result<Output, std::io::Error> {
        self.app
            .args(args)
            .current_dir(working_directory)
            .envs(env_vars)
            .output()
    }
}
