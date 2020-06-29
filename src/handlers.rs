use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str;

#[derive(Debug, Deserialize)]
pub struct AppInput {
    pub arguments: String,
    pub working_directory: PathBuf,
    pub env_vars: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct AppOutput {
    stdout: String,
    stderr: String,
    success: bool,
}

impl AppOutput {
    pub fn new(stdout: Vec<u8>, stderr: Vec<u8>, success: bool) -> AppOutput {
        AppOutput {
            stdout: str::from_utf8(&stdout).unwrap().to_owned(),
            stderr: str::from_utf8(&stderr).unwrap().to_owned(),
            success,
        }
    }
}
