use bitflags::bitflags;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

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
    walk: Walk,
    prev_power: Power,
    current_power: Power,
}

impl Controls {
    pub(crate) fn new() -> Self {
        Self {
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
        // TODO: KeyboardInput events have strange repeat patterns.

        let dir = Direction::from_bits(match self.walk {
            Walk::NoInput => 0,
            Walk::Walk(dir) => dir.bits,
        })
        .expect("No direction to decode");

        if let Some(VirtualKeyCode::W) = key.virtual_keycode {
            if key.state == ElementState::Pressed {
                self.walk = Walk::Walk((dir - Direction::DOWN) | Direction::UP);
            } else {
                self.walk = Walk::Walk(dir - Direction::UP);
            }
        } else if let Some(VirtualKeyCode::A) = key.virtual_keycode {
            if key.state == ElementState::Pressed {
                self.walk = Walk::Walk((dir - Direction::RIGHT) | Direction::LEFT);
            } else {
                self.walk = Walk::Walk(dir - Direction::LEFT);
            }
        } else if let Some(VirtualKeyCode::S) = key.virtual_keycode {
            if key.state == ElementState::Pressed {
                self.walk = Walk::Walk((dir - Direction::UP) | Direction::DOWN);
            } else {
                self.walk = Walk::Walk(dir - Direction::DOWN);
            }
        } else if let Some(VirtualKeyCode::D) = key.virtual_keycode {
            if key.state == ElementState::Pressed {
                self.walk = Walk::Walk((dir - Direction::LEFT) | Direction::RIGHT);
            } else {
                self.walk = Walk::Walk(dir - Direction::RIGHT);
            }
        } else if let Some(VirtualKeyCode::Space) = key.virtual_keycode {
            self.prev_power = self.current_power;
            if key.state == ElementState::Pressed {
                self.current_power = Power::Use;
            } else {
                self.current_power = Power::NoInput;
            }
        } else if let Some(VirtualKeyCode::Tab) = key.virtual_keycode {
            self.prev_power = self.current_power;
            if key.state == ElementState::Pressed {
                self.current_power = Power::Select;
            } else {
                self.current_power = Power::NoInput;
            }
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
