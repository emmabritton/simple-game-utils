//! Simple Game Utilities
//!
//! # Usage
//!
//! ```no_run
//!# const BYTES: [u8; 7] = [0,0,0,0,0,0,0];
//!# fn main() {
//!# use simple_game_utils::prelude::*;
//!# let engine = AudioEngine::new().unwrap();
//!# let duration = 1.0;
//! let mut timing = Timing::new(240);
//! let mut timer = Timer::new_with_delay(1.0, 1.0); //timer that triggers after 1s then every second
//! let mut sound = engine.load_from_bytes(&BYTES, duration).unwrap();
//!
//! sound.play();
//! loop {
//! println!("Other");
//!     sound.update(&timing);
//!     if timer.update(&timing) {
//!         break;
//!     }
//! }
//!# }
//! ```

#[cfg(feature = "controller")]
pub mod controller;
pub mod error;
#[cfg(feature = "sound")]
pub mod sound_effect;
pub mod timing;
#[cfg(feature = "prefs")]
pub mod prefs;

pub mod prelude {
    #[cfg(feature = "controller")]
    pub use crate::controller::*;
    pub use crate::error::*;
    #[cfg(feature = "sound")]
    pub use crate::sound_effect::*;
    pub use crate::timing::*;
    #[cfg(feature = "sound")]
    pub use audio_engine::AudioEngine;
    #[cfg(feature = "prefs")]
    pub use crate::prefs::*;
    #[cfg(feature = "prefs")]
    pub use crate::prefs::app_prefs::*;
}
