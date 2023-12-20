use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameUtilError {
    #[cfg(feature = "sound")]
    #[error("Error init'ing sound effect: {0}")]
    SoundEffectInit(&'static str),
    #[cfg(feature = "sound")]
    #[error("Invalid sound data")]
    SoundEffectInvalid(hound::Error),
}
