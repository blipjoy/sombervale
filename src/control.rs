use bitflags::bitflags;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use winit::event::{ElementState, KeyboardInput};

// Key map for Windows and Linux: http://flint.cs.yale.edu/cs422/doc/art-of-asm/pdf/APNDXC.PDF
#[cfg(not(target_os = "macos"))]
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u32)]
enum KeyMap {
    W = 17,
    A = 30,
    S = 31,
    D = 32,
    Space = 57,
    Tab = 15,
}

// Keymap for macOS: https://bit.ly/3kThGwO
#[cfg(target_os = "macos")]
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u32)]
enum KeyMap {
    W = 13,
    A = 0,
    S = 1,
    D = 2,
    Space = 49,
    Tab = 48,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Walk {
    NoInput,
    Walk(Direction),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Power {
    NoInput,
    Use,
    Select,
}

#[derive(Copy, Clone, Debug, Default)]
struct Keys {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
    tab: bool,
    space: bool,
}

impl Keys {
    fn update(&mut self, key: KeyboardInput) {
        match KeyMap::try_from(key.scancode) {
            Ok(KeyMap::W) => {
                self.w = key.state == ElementState::Pressed;
            }
            Ok(KeyMap::A) => {
                self.a = key.state == ElementState::Pressed;
            }
            Ok(KeyMap::S) => {
                self.s = key.state == ElementState::Pressed;
            }
            Ok(KeyMap::D) => {
                self.d = key.state == ElementState::Pressed;
            }
            Ok(KeyMap::Space) => {
                self.space = key.state == ElementState::Pressed;
            }
            Ok(KeyMap::Tab) => {
                self.tab = key.state == ElementState::Pressed;
            }
            // Ignore everything else
            _ => {}
        }
    }
}

bitflags! {
    pub(crate) struct Direction: u8 {
        const UP = 0b0001;
        const DOWN = 0b0010;
        const LEFT = 0b0100;
        const RIGHT = 0b1000;

        const UP_LEFT = Self::UP.bits | Self::LEFT.bits;
        const UP_RIGHT = Self::UP.bits | Self::RIGHT.bits;
        const DOWN_LEFT = Self::DOWN.bits | Self::LEFT.bits;
        const DOWN_RIGHT = Self::DOWN.bits | Self::RIGHT.bits;
    }
}

pub(crate) struct Controls {
    keys: Keys,
    walk: Walk,
    prev_power: Power,
    current_power: Power,
}

impl Controls {
    pub(crate) fn new() -> Self {
        Self {
            keys: Keys::default(),
            walk: Walk::NoInput,
            prev_power: Power::NoInput,
            current_power: Power::NoInput,
        }
    }

    pub(crate) fn walk(&self) -> Walk {
        self.walk
    }

    pub(crate) fn power(&mut self) -> Power {
        if self.prev_power != self.current_power {
            self.prev_power = self.current_power;
            self.prev_power
        } else {
            Power::NoInput
        }
    }

    pub(crate) fn update(&mut self, key: KeyboardInput) {
        // Capture all key states
        self.keys.update(key);

        // Reset actions states
        self.walk = Walk::NoInput;
        self.prev_power = self.current_power;
        self.current_power = Power::NoInput;

        // Translate key states into actions
        let mut dir = Direction::from_bits(match self.walk {
            Walk::NoInput => 0,
            Walk::Walk(dir) => dir.bits,
        })
        .expect("No direction to decode");

        if self.keys.w {
            dir = (dir - Direction::DOWN) | Direction::UP;
            self.walk = Walk::Walk(dir);
        }
        if self.keys.a {
            dir = (dir - Direction::RIGHT) | Direction::LEFT;
            self.walk = Walk::Walk(dir);
        }
        if self.keys.s {
            dir = (dir - Direction::UP) | Direction::DOWN;
            self.walk = Walk::Walk(dir);
        }
        if self.keys.d {
            dir = (dir - Direction::LEFT) | Direction::RIGHT;
            self.walk = Walk::Walk(dir);
        }
        if self.keys.space {
            self.current_power = Power::Use;
        }
        if self.keys.tab {
            self.current_power = Power::Select;
        }

        // Never end up with Walk::Walk(0)
        let dir = match self.walk {
            Walk::NoInput => 0,
            Walk::Walk(dir) => dir.bits,
        };
        if dir == 0 {
            self.walk = Walk::NoInput;
        }
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self::new()
    }
}
