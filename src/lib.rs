//! Simple Game Utilities
//!
//! # Usage
//!
//! ```
//!# use simple_game_utils::prelude::*;
//!# let engine = AudioEngine::new().unwrap();
//!# let bytes = [0,0,0,0,0,0,0];
//!# let duration = 1.0;
//! let mut  timing = Timing::new(240);
//! let mut  timer = Timer::new_with_delay(1.0, 1.0); //timer that triggers after 1s then every second
//! let mut  sound = engine.load_from_bytes(&bytes, duration).unwrap();
//!
//! sound.play();
//! loop {
//!     timing.update();
//!     sound.update(&timing);
//!     if timer.update(&timing) {
//!         break;
//!     }
//! }
//!
//! ```

#[cfg(feature = "controller")]
pub mod controller;
pub mod error;
#[cfg(feature = "sound")]
pub mod sound_effect;
pub mod timing;

pub mod prelude {
    #[cfg(feature = "controller")]
    pub use crate::controller::*;
    pub use crate::error::*;
    #[cfg(feature = "sound")]
    pub use crate::sound_effect::*;
    pub use crate::timing::*;
    #[cfg(feature = "sound")]
    pub use audio_engine::AudioEngine;
}
