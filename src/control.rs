use bitflags::bitflags;
use winit::event::{ElementState, KeyboardInput};

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

        // W = 17
        if key.scancode == 17 {
            if key.state == ElementState::Pressed {
                self.walk = Walk::Walk(dir | Direction::UP);
            } else {
                self.walk = Walk::Walk(dir - Direction::UP);
            }
        }
        // A = 30
        else if key.scancode == 30 {
            if key.state == ElementState::Pressed {
                self.walk = Walk::Walk(dir | Direction::LEFT);
            } else {
                self.walk = Walk::Walk(dir - Direction::LEFT);
            }
        }
        // S = 31
        else if key.scancode == 31 {
            if key.state == ElementState::Pressed {
                self.walk = Walk::Walk(dir | Direction::DOWN);
            } else {
                self.walk = Walk::Walk(dir - Direction::DOWN);
            }
        }
        // D = 32
        else if key.scancode == 32 {
            if key.state == ElementState::Pressed {
                self.walk = Walk::Walk(dir | Direction::RIGHT);
            } else {
                self.walk = Walk::Walk(dir - Direction::RIGHT);
            }
        }
        // Space = 57
        else if key.scancode == 57 {
            self.prev_power = self.current_power;
            if key.state == ElementState::Pressed {
                self.current_power = Power::Use;
            } else {
                self.current_power = Power::NoInput;
            }
        }
        // Tab = 15
        else if key.scancode == 15 {
            self.prev_power = self.current_power;
            if key.state == ElementState::Pressed {
                self.current_power = Power::Select;
            } else {
                self.current_power = Power::NoInput;
            }
        }

        // Avoid opposite directions
        if let Walk::Walk(dir) = &mut self.walk {
            let up_down = Direction::UP | Direction::DOWN;
            if *dir & up_down == up_down {
                *dir -= Direction::UP | Direction::DOWN;
            }

            let left_right = Direction::LEFT | Direction::RIGHT;
            if *dir & left_right == left_right {
                *dir -= Direction::LEFT | Direction::RIGHT;
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
