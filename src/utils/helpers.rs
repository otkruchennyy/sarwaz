use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Failed to create directory: {0}")]
    DirectoryCreation(String),
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn create_empty_json(target_dir: &Path) -> Result<PathBuf, ProjectError> {
    let file_path = target_dir.join("data.json");
    fs::write(&file_path, serde_json::to_string_pretty(&json!({}))?)?;
    Ok(file_path)
}

pub struct ProjectManager {
    project_root: PathBuf,
    data_file: PathBuf,
}

impl ProjectManager {
    pub fn new() -> Result<Self, ProjectError> {
        let project_root = Self::find_or_create_project_root()?;
        Ok(Self {
            data_file: project_root.join("data.json"),
            project_root,
        })
    }

    fn find_or_create_project_root() -> Result<PathBuf, ProjectError> {
        if let Ok(env_path) = env::var("SARWAZ_ROOT") {
            let path = PathBuf::from(env_path);
            if path.exists() {
                Self::ensure_assets_dir(&path)?;
                return Ok(path);
            }
        }

        let current_dir = env::current_dir()?;
        Self::find_project_root_from(&current_dir)
            .or_else(|| Self::find_sarwaz_folder(&current_dir))
            .map(|p| {
                Self::ensure_assets_dir(&p).ok();
                p
            })
            .or_else(|| {
                Self::ensure_assets_dir(&current_dir).ok();
                Some(current_dir)
            })
            .ok_or_else(|| {
                ProjectError::DirectoryCreation("Не удалось определить корень проекта".into())
            })
    }

    fn find_project_root_from(start: &Path) -> Option<PathBuf> {
        let mut current = start.to_path_buf();
        while let Some(parent) = current.parent() {
            if current.join("assets").exists() {
                return Some(current);
            }
            current = parent.to_path_buf();
        }
        None
    }

    fn find_sarwaz_folder(start: &Path) -> Option<PathBuf> {
        let mut current = start.to_path_buf();
        while let Some(parent) = current.parent() {
            if let Some(name) = current.file_name() {
                if name.to_string_lossy().to_lowercase().contains("sarwaz") {
                    return Some(current);
                }
            }
            current = parent.to_path_buf();
        }
        None
    }

    fn ensure_assets_dir(project_root: &Path) -> Result<(), ProjectError> {
        let assets_dir = project_root.join("assets");
        if !assets_dir.exists() {
            fs::create_dir_all(&assets_dir).map_err(|e| {
                ProjectError::DirectoryCreation(format!("{}: {}", assets_dir.display(), e))
            })?;
        }
        Ok(())
    }

    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    pub fn data_file(&self) -> &Path {
        &self.data_file
    }

    pub fn data_file_exists(&self) -> bool {
        self.data_file.is_file()
    }

    pub fn add_to_json(&self, key: &str, value: Value) -> Result<(), ProjectError> {
        let mut data: Value = if self.data_file_exists() {
            serde_json::from_str(&fs::read_to_string(&self.data_file)?)?
        } else {
            json!({})
        };

        data[key] = value;
        fs::write(&self.data_file, serde_json::to_string_pretty(&data)?)?;
        Ok(())
    }

    pub fn remove_from_json(&self, key: &str) -> Result<(), ProjectError> {
        if !self.data_file_exists() {
            return Ok(());
        }

        let mut data: Value = serde_json::from_str(&fs::read_to_string(&self.data_file)?)?;
        if let Some(obj) = data.as_object_mut() {
            obj.remove(key);
        }

        fs::write(&self.data_file, serde_json::to_string_pretty(&data)?)?;
        Ok(())
    }
}

pub fn get_project_root() -> PathBuf {
    let mut current = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    loop {
        if current.join("assets").exists() {
            return current;
        }

        if let Some(name) = current.file_name() {
            if name.to_string_lossy().to_lowercase().contains("sarwaz") {
                let assets_dir = current.join("assets");
                if !assets_dir.exists() {
                    let _ = fs::create_dir_all(&assets_dir);
                }
                return current;
            }
        }

        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => break,
        }
    }

    let assets_dir = env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("assets");

    let _ = fs::create_dir_all(&assets_dir);
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
