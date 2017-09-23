use serde_yaml;

use std::env::home_dir;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Seek, SeekFrom, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Crate {
    pub name: String,
    pub downloads: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub crates: Vec<Crate>,
}

impl Config {
    pub fn new() -> serde_yaml::Result<Self> {
        let home_dir = home_dir().unwrap();
        let mut config_dir_path = home_dir.clone();
        config_dir_path.push(".config");
        config_dir_path.push("crate-installs-notifier");
        let mut config_file_path = config_dir_path.clone();
        config_file_path.push("config.yaml");

        let config_file = if let Ok(c) = File::open(&config_file_path) {
            c
        } else {
            create_dir_all(config_dir_path)?;
            let mut config_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&config_file_path)?;
            let config = serde_yaml::to_string(&Config { crates: vec![] }).unwrap();
            config_file.write_all(config.as_bytes())?;
            config_file.seek(SeekFrom::Start(0))?;
            config_file
        };

        serde_yaml::from_reader(config_file)
    }

    pub fn update(&self) -> serde_yaml::Result<()> {
        let home_dir = home_dir().unwrap();
        let mut config_dir_path = home_dir.clone();
        config_dir_path.push(".config");
        config_dir_path.push("crate-installs-notifier");
        let mut config_file_path = config_dir_path.clone();
        config_file_path.push("config.yaml");
        let mut config_file = File::create(&config_file_path)?;
        let config_buf = serde_yaml::to_vec(self)?;
        config_file.write_all(&config_buf)?;

        Ok(())
    }
}
