use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub library: LibraryConfig,
    pub sample: SampleConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryConfig {
    pub project_path: PathBuf,
    pub module_name: String,
    pub build_variant: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SampleConfig {
    pub project_path: PathBuf,
    pub libs_path: PathBuf,
    pub build_variant: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read config file")?;
        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn aar_source_path(&self) -> PathBuf {
        self.library.project_path
            .join("build/outputs/aar")
            .join(format!("{}-{}.aar", self.library.module_name, self.library.build_variant))
    }

    pub fn aar_dest_path(&self) -> PathBuf {
        self.sample.project_path
            .join(&self.sample.libs_path)
            .join(format!("{}.aar", self.library.module_name))
    }
}