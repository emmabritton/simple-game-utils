//! Simple Game Utilities
//!
//! # Usage
//!
//! ```no_run
//!# const BYTES: [u8; 7] = [0,0,0,0,0,0,0];
//!# const TILESET_FILE_CONTENTS: &str = "";
//!# const TILEMAP_FILE_CONTENTS: &str = "";
//!# fn main() {
//!# use ici_files::image::IndexedImage;
//!# use simple_game_utils::prelude::*;
//!# let engine = AudioEngine::new().unwrap();
//!# let duration = 1.0;
//!# struct Graphics {
//!# }
//!# impl Graphics {
//!#     fn draw_indexed_image(&self, pos: (isize,isize), img: &IndexedImage) {}
//!# }
//! let mut timing = Timing::new(240);
//! let mut timer = Timer::new_with_delay(1.0, 2.0); //timer that triggers every second after waiting 2s initially
//! let mut sound = engine.load_from_bytes(&BYTES, duration).unwrap();
//! let ici_tileset: IciTileset = ron::from_str(TILESET_FILE_CONTENTS).unwrap();
//! let tileset = ici_tileset.into_tileset().unwrap();
//! let tilemap_file: TilemapFile = ron::from_str(TILEMAP_FILE_CONTENTS).unwrap();
//! let tilemap: Tilemap<IndexedImage> = tilemap_file.into_tilemap(&tileset, (200,200)).unwrap();
//!
//! sound.play();
//! loop {
//!     sound.update(&timing);
//!     if timer.update(&timing) {
//!         break;
//!     }
//! }
//! # let graphics = Graphics{};
//! tilemap.draw(|img, pos| graphics.draw_indexed_image(pos, img));
//!# }
//! ```

#[cfg(feature = "controller")]
pub mod controller;
pub mod error;
#[cfg(feature = "prefs")]
pub mod prefs;
#[cfg(feature = "sound")]
pub mod sound_effect;
pub mod tiles;
pub mod timing;

pub mod prelude {
    #[cfg(feature = "controller")]
    pub use crate::controller::*;
    pub use crate::error::*;
    #[cfg(feature = "prefs")]
    pub use crate::prefs::app_prefs::*;
    #[cfg(feature = "prefs")]
    pub use crate::prefs::*;
    #[cfg(feature = "sound")]
    pub use crate::sound_effect::*;
    pub use crate::tiles::prelude::*;
    pub use crate::timing::*;
    #[cfg(feature = "sound")]
    pub use audio_engine::AudioEngine;
}
