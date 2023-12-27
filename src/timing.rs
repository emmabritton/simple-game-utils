#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Used for single or repeated timed events, uses fractional seconds
///
/// # Usage
///
/// ```
///# use simple_game_utils::timing::Timer;
///# let delta = 0.1;
///let mut  timer = Timer::new_with_delay(1.0, 1.0);
///loop {
///    if timer.update_secs(delta) {
///         break;
///    }
///}
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Timer {
    /// amount of time remaining
    remaining: f64,
    /// amount of time to reset to once `remaining` <= 0
    reset: f64,
    /// if the timer should automatically reset
    looping: bool,
}

impl Timer {
    /// Create a timer with the duration of `reset` that automatically resets after triggering
    /// Will trigger after `after`
    pub fn new_with_delay(after: f64, reset: f64) -> Self {
        Self {
            remaining: after,
            reset,
            looping: true,
        }
    }

    /// Create a timer with the duration of `reset` that automatically resets after triggering
    /// Will trigger immediately
    pub fn new(reset: f64) -> Self {
        Self {
            remaining: 0.0,
            reset,
            looping: true,
        }
    }

    /// Create a timer with the duration of `reset` that only triggers once
    pub fn new_once(after: f64) -> Self {
        Self {
            remaining: after,
            reset: after,
            looping: false,
        }
    }
}

impl Timer {
    /// Update timer, returns true if triggered
    pub fn update(&mut self, timing: &Timing) -> bool {
        self.update_secs(timing.fixed_time_step)
    }

    /// Update timer, returns true if triggered
    /// `delta` is fractional seconds passed since last call
    pub fn update_secs(&mut self, delta: f64) -> bool {
        self.remaining -= delta;
        let triggered = self.remaining <= 0.0;
        if triggered && self.looping {
            self.remaining = self.reset;
        }
        triggered
    }

    /// Set remaining to reset value (the number passed into the constructor)
    /// One time loops can trigger again after calling this
    pub fn reset(&mut self) {
        self.remaining = self.reset;
    }

    /// If the timer has reached 0, this will always be false for looping timers (unless reset is <= 0.0)
    pub fn has_triggered(&self) -> bool {
        self.remaining <= 0.0
    }

    /// Set remaining to 0, triggering the timer
    pub fn trigger(&mut self) {
        self.remaining = 0.0;
    }

    /// Add `seconds` to remaining
    pub fn delay(&mut self, seconds: f64) {
        self.remaining += seconds;
    }
}

/// Used to track time in games
#[derive(Debug, Clone, PartialEq)]
pub struct Timing {
    /// amount of time that has passed since last
    pub delta: f64,
    /// when execution started
    pub started_at: Instant,
    /// time at start of frame
    pub now: Instant,
    /// time at start of last frame
    pub last: Instant,
    /// number of updates so far
    pub updates: usize,
    /// number of renders so far
    pub renders: usize,
    pub accumulated_time: f64,
    max_render_time: f64,
    /// an fps independent value used to update animations, etc
    pub fixed_time_step: f64,
    /// an fps independent value used to update animations, etc
    pub fixed_time_step_f32: f32,
    /// FPS
    pub stats: Stats,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stats {
    /// The number of frames shown in the last second
    pub fps: usize,
    /// Used to calculate fps
    pub last_frame_count: usize,
    /// Used to calculate fps
    pub last_frame_check: Instant,
}

impl Timing {
    pub fn new(speed: usize) -> Timing {
        Timing {
            delta: 0.0,
            started_at: Instant::now(),
            now: Instant::now(),
            last: Instant::now(),
            updates: 0,
            renders: 0,
            accumulated_time: 0.0,
            max_render_time: 0.1,
            fixed_time_step: 1.0 / (speed as f64),
            fixed_time_step_f32: 1.0 / (speed as f32),
            stats: Stats {
                fps: 0,
                last_frame_count: 0,
                last_frame_check: Instant::now(),
            },
        }
    }

    pub fn update_fps(&mut self) {
        if self
            .now
            .duration_since(self.stats.last_frame_check)
            .as_secs_f32()
            >= 1.0
        {
            self.stats.fps = self.renders - self.stats.last_frame_count;
            self.stats.last_frame_check = self.now;
            self.stats.last_frame_count = self.renders;
        }
    }

    pub fn update(&mut self) {
        self.now = Instant::now();
        self.delta = self.now.duration_since(self.last).as_secs_f64();
        self.accumulated_time += self.delta;
        if self.delta > self.max_render_time {
            self.delta = self.max_render_time;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::timing::Timer;

    #[test]
    fn basic_test_delayed() {
        let mut timer = Timer::new_with_delay(1.0, 1.0);
        assert!(!timer.update_secs(0.4));
        assert!(!timer.update_secs(0.4));
        assert!(timer.update_secs(0.4));
        assert!(!timer.has_triggered());
    }

    #[test]
    fn basic_test() {
        let mut timer = Timer::new(0.5);
        assert!(timer.has_triggered());
        timer.reset();
        assert!(!timer.has_triggered());
        assert!(!timer.update_secs(0.4));
        assert!(timer.update_secs(0.4));
        assert!(!timer.has_triggered());
    }
}
