use std::{fs::File, path::PathBuf};
use std::fmt;
use std::error::Error;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyPath {
    pub from: String,
    pub to: String,
    pub recursive: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Step {
    Link(CopyPath),
    Copy(CopyPath),
    Shell(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stage {
    pub name: Option<String>,
    pub steps: Option<Vec<Step>>,
    pub from_file: Option<String>,

    #[serde(skip_deserializing)] 
    pub base_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub stages: Vec<Stage>,

    #[serde(skip_deserializing)] 
    pub base_path: PathBuf,
}

#[derive(Debug)]
struct StageFileError {
    stage_file: String,
    cause: Box<dyn std::error::Error>,
}

impl Error for StageFileError {}

impl fmt::Display for StageFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stage_file = &self.stage_file;
        let cause = &self.cause.to_string();

        write!(f, "Failed to load stage from file '{stage_file}': {cause}")
    }
}

impl Config {
    fn resolve_stage_file(base_path: PathBuf, file_name: &str) -> Result<Stage, Box<dyn std::error::Error>> {
        let mut file_path = base_path.clone();
        file_path.push(file_name);
        let file = File::open(&file_path)?;
        let stage: Stage = serde_yaml::from_reader(file)?;
        let parent_path = file_path.parent().unwrap();

        Ok(Stage { base_path: parent_path.to_path_buf(), ..stage })
    }

    pub fn from_file(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let base_path = std::env::current_dir()?;
        let mut file_path = base_path.clone();
        file_path.push(file_name);

        let file = File::open(&file_path)?;
        let loaded_config: Config = serde_yaml::from_reader(file)?;
        
        let mut stages: Vec<Stage> = Vec::new();
        
        for stage in loaded_config.stages {
            if let Some(file_name) = stage.from_file {
                match Self::resolve_stage_file(base_path.clone(), &file_name) {
                    Ok(loaded_stage) => stages.push(loaded_stage),
                    Err(err) => return Err(Box::new(StageFileError {
                        stage_file: file_name,
                        cause: err
                    }))
                }
            } else {
                stages.push(Stage { base_path: base_path.clone(), ..stage });
            }
        }
        
        Ok(Config { stages, base_path: base_path })
    }
}