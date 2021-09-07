use std::time::{Duration, Instant};

pub(crate) struct FrogPower {
    // Experience points
    max_xp: usize,
    xp: usize,

    // Power points (not a presentation)
    max_pp: usize,
    pp: usize,

    cooldown: Duration,
    start: Instant,
}

impl FrogPower {
    fn new() -> Self {
        Self {
            max_xp: 2,
            xp: 0,
            max_pp: 1,
            pp: 1,
            cooldown: Duration::from_secs(3),
            start: Instant::now(),
        }
    }

    pub(crate) fn update(&mut self, frogs: usize) {
        // Increase the power meter when the number of live frogs is less than the player's pp
        if self.pp < self.max_pp
            && frogs < self.max_pp - self.pp
            && self.start.elapsed() >= self.cooldown
        {
            self.start = Instant::now();
            self.pp += 1;
        }
    }

    pub(crate) fn use_power(&mut self) -> bool {
        if self.pp > 0 {
            // Reset cooldown only when the meter is full
            if self.pp == self.max_pp {
                self.start = Instant::now();
            }

            self.pp -= 1;

            true
        } else {
            false
        }
    }

    pub(crate) fn xp(&self) -> usize {
        self.xp
    }

    pub(crate) fn max_xp(&self) -> usize {
        self.max_xp
    }

    pub(crate) fn pp(&self) -> usize {
        self.pp
    }

    pub(crate) fn max_pp(&self) -> usize {
        self.max_pp
    }

    pub(crate) fn increase_xp(&mut self) {
        if self.xp < self.max_xp {
            self.xp += 1;
        } else {
            self.max_pp += 1;
            self.max_xp *= 2;
            self.xp = 0;
        }
    }
}

impl Default for FrogPower {
    fn default() -> Self {
        Self::new()
    }
}
