pub mod app_prefs;

use crate::error::GameUtilError;
use directories::ProjectDirs;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::remove_file;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Preferences<T>
where
    T: Serialize + DeserializeOwned,
{
    data: HashMap<String, T>,
    file: PathBuf,
}

impl<T: Serialize + DeserializeOwned> Preferences<T> {
    pub fn new(mut path: PathBuf, filename: &str) -> Self {
        path.push(filename);
        Preferences {
            data: HashMap::new(),
            file: path,
        }
    }
}

impl<T: Serialize + DeserializeOwned> Preferences<T> {
    pub fn load(&mut self) -> Result<(), GameUtilError> {
        let json_str = if self.file.exists() {
            fs::read_to_string(&self.file).map_err(|err| {
                GameUtilError::Loading(err.to_string(), self.file.to_string_lossy().to_string())
            })?
        } else {
            String::from("{}")
        };
        self.data = serde_json::from_str(&json_str)
            .map_err(|err| GameUtilError::Deserializing(err.to_string()))?;

        Ok(())
    }

    pub fn save(&self) -> Result<(), GameUtilError> {
        let json_str = serde_json::to_string(&self.data)
            .map_err(|err| GameUtilError::Serializing(err.to_string()))?;
        fs::write(&self.file, json_str).map_err(|err| {
            GameUtilError::Saving(err.to_string(), self.file.to_string_lossy().to_string())
        })?;

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: &str, value: T) {
        self.data.insert(key.to_owned(), value);
    }

    pub fn clear(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn delete_file(&self) -> bool {
        remove_file(&self.file).is_ok()
    }
}

pub fn get_pref_dir(
    qualifier: &str,
    organization: &str,
    application: &str,
) -> Result<PathBuf, GameUtilError> {
    if let Some(dir) = ProjectDirs::from(qualifier, organization, application) {
        let path = dir.preference_dir().to_path_buf();
        fs::create_dir_all(path.clone()).map_err(|err| {
            GameUtilError::MakingDirs(err.to_string(), path.to_string_lossy().to_string())
        })?;
        Ok(path)
    } else {
        Err(GameUtilError::AppPrefDir)
    }
}
