use crate::error::GameUtilError;
use crate::timing::Timing;
use audio_engine::{AudioEngine, Sound, WavDecoder};
use std::fmt::{Debug, Formatter};
use std::io::Cursor;

/// Sound effect (although it can also be used for music)
/// You must call [SoundEffect::update] or [SoundEffect::update_secs] with accurate values and often otherwise playback may stutter or jump
///
/// # Usage
///
/// ```no_run
///# const BYTES: [u8; 6] = [0,0,0,0,0,0];
///# fn main() {
///# use audio_engine::AudioEngine;
///# use simple_game_utils::sound_effect::NewSoundEffect;
///# use simple_game_utils::timing::Timing;
///# let mut  timing = Timing::new(240);
///# let duration = 1.0;
/// //this must live as long as `sound` but there's no lifetimes to enforce this
/// let mut engine = AudioEngine::new().unwrap();
/// let mut sound = engine.load_from_bytes(&BYTES, duration).unwrap();
/// sound.play();
/// loop {
///     timing.update();
///     sound.update(&timing);
/// }
///# }
/// ```
pub struct SoundEffect {
    //Sound data
    sound: Sound,
    //If sound is currently playing
    is_playing: bool,
    //Length in seconds
    duration: f64,
    // used to prevent bugs
    next_play_in: f64,
    //If sound automatically loops
    loops: bool,
}

impl Debug for SoundEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sound: is_playing: {}, duration: {:.1}s, loops: {}",
            self.is_playing, self.duration, self.loops
        )
    }
}

pub trait NewSoundEffect {
    fn load_from_bytes(
        &self,
        bytes: &'static [u8],
        duration: f64,
    ) -> Result<SoundEffect, GameUtilError>;
}

impl NewSoundEffect for AudioEngine {
    fn load_from_bytes(
        &self,
        bytes: &'static [u8],
        duration: f64,
    ) -> Result<SoundEffect, GameUtilError> {
        let decoder =
            WavDecoder::new(Cursor::new(bytes)).map_err(GameUtilError::SoundEffectInvalid)?;
        let sound = self
            .new_sound(decoder)
            .map_err(GameUtilError::SoundEffectInit)?;
        Ok(SoundEffect::new(sound, duration))
    }
}

impl SoundEffect {
    pub fn new(sound: Sound, duration: f64) -> Self {
        Self {
            sound,
            is_playing: false,
            duration,
            next_play_in: 0.0,
            loops: false,
        }
    }

    /// Play sound effect, won't do anything if sound effect is already playing
    pub fn play(&mut self) {
        if !self.is_playing {
            self.sound.play();
            self.is_playing = true;
            self.next_play_in = self.duration;
        }
    }

    /// Reset playback position and stop playback
    pub fn reset(&mut self) {
        self.sound.stop();
        self.is_playing = false;
        self.next_play_in = 0.0;
    }

    /// Set if the sound loops automatically
    pub fn set_loop(&mut self, loops: bool) {
        self.loops = loops;
        self.sound.set_loop(loops)
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sound.set_volume(volume);
    }

    /// Returns true if calling [SoundEffect::play] will do anything
    pub fn can_play(&self) -> bool {
        !self.is_playing && self.next_play_in < 0.0
    }

    /// Allows the sound to continue playing
    pub fn update(&mut self, timing: &Timing) {
        self.update_secs(timing.fixed_time_step)
    }

    /// Allows the sound to continue playing
    pub fn update_secs(&mut self, delta: f64) {
        if !self.loops && self.is_playing && self.next_play_in < 0.0 {
            self.reset();
        }
        self.next_play_in -= delta;
    }

    /// If the sound is currently playing
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Length in seconds
    pub fn duration(&self) -> f64 {
        self.duration
    }

    /// If sound will automatically loop
    pub fn loops(&self) -> bool {
        self.loops
    }
}
