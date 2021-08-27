use std::time::{Duration, Instant};

pub(crate) struct FrogPower {
    max_level: usize,
    level: usize,
    cooldown: Duration,
    start: Instant,
}

impl FrogPower {
    fn new() -> Self {
        Self {
            max_level: 1,
            level: 1,
            cooldown: Duration::from_secs(1),
            start: Instant::now(),
        }
    }

    pub(crate) fn update(&mut self, frogs: usize) {
        // Increase the power meter when the number of live frogs is less than the player's level
        if self.level < self.max_level
            && frogs < self.max_level - self.level
            && self.start.elapsed() >= self.cooldown
        {
            self.start = Instant::now();
            self.level += 1;
        }
    }

    pub(crate) fn use_power(&mut self) -> bool {
        if self.level > 0 {
            // Reset cooldown only when the meter is full
            if self.level == self.max_level {
                self.start = Instant::now();
            }

            self.level -= 1;

            true
        } else {
            false
        }
    }

    pub(crate) fn level(&self) -> usize {
        self.level
    }

    pub(crate) fn max_level(&self) -> usize {
        self.max_level
    }
}

impl Default for FrogPower {
    fn default() -> Self {
        Self::new()
    }
}
