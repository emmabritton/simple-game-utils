use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameUtilError {
    #[cfg(feature = "sound")]
    #[error("Error init'ing sound effect: {0}")]
    SoundEffectInit(&'static str),
    #[cfg(feature = "sound")]
    #[error("Invalid sound data")]
    SoundEffectInvalid(hound::Error),
    #[cfg(feature = "prefs")]
    #[error("Unable to get app pref dir")]
    AppPrefDir,
    #[cfg(feature = "prefs")]
    #[error("Saving prefs: {0} to {1}")]
    Saving(String, String),
    #[cfg(feature = "prefs")]
    #[error("Serializing data: {0}")]
    Serializing(String),
    #[cfg(feature = "prefs")]
    #[error("Loading prefs: {0} from {1}")]
    Loading(String, String),
    #[cfg(feature = "prefs")]
    #[error("Deserializing data: {0}")]
    Deserializing(String),
    #[cfg(feature = "prefs")]
    #[error("Creating pref dir: {0} at {1}")]
    MakingDirs(String, String),
    #[error("Tileset for {0} is missing images: {1:?}")]
    InvalidTileset(String, Vec<String>),
}
