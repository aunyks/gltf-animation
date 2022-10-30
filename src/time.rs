use std::cmp::{PartialEq, PartialOrd};
use std::time::Duration;

/// A clock that passes time whenever it's
/// ticked. This clock is great for pure simulation
/// timekeeping as it doesn't need access to system-native
/// clocks and only ticks through the provided `tick()` function.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassiveClock(Duration);

impl PassiveClock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        self.0 += Duration::from_secs_f32(delta_seconds);
    }

    pub fn reset(&mut self) {
        self.0 = Duration::default();
    }

    pub fn elapsed(&self) -> Duration {
        self.0
    }
}
