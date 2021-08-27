use crate::animation::Animated;
use crate::image::Image;
use crate::map::Rect;
use crate::power::FrogPower;
use getrandom::getrandom;
use randomize::PCG32;
use shipyard::EntityId;
use std::convert::TryInto;
use std::time::Instant;
use ultraviolet::{Vec2, Vec3};

pub(crate) struct Random(pub(crate) PCG32);
pub(crate) struct Controls(pub(crate) crate::control::Controls);
pub(crate) struct UpdateTime(pub(crate) Instant);
pub(crate) struct Position(pub(crate) Vec3);
pub(crate) struct Velocity(pub(crate) Vec3);
pub(crate) struct Animation<A: Animated>(pub(crate) A);
pub(crate) struct Annihilate(pub(crate) Vec<EntityId>);

pub(crate) struct Viewport {
    pub(crate) pos: Vec2,
    pub(crate) world_height: f32,
}

pub(crate) struct Collision {
    pub(crate) shapes: Vec<Rect>,
}

pub(crate) struct Tilemap {
    pub(crate) image: Image,
    pub(crate) parallax: Vec2,
}

pub(crate) struct Sprite {
    pub(crate) image: Image,
    pub(crate) frame_height: isize,
    pub(crate) frame_index: usize,
}

pub(crate) struct Follow {
    pub(crate) entity_id: EntityId,
    pub(crate) direction: Vec3,
}

#[derive(Default)]
pub(crate) struct Hud {
    pub(crate) frog_power: Option<FrogPower>,
}

pub(crate) struct Intro {}

impl Default for Random {
    fn default() -> Self {
        let mut seed = [0_u8; 16];

        getrandom(&mut seed).expect("failed to getrandom");
        let inc = u64::from_ne_bytes(seed[0..8].try_into().unwrap());
        let seed = u64::from_ne_bytes(seed[8..16].try_into().unwrap());

        Self(PCG32::seed(seed, inc))
    }
}

impl Random {
    pub(crate) fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    /// Random f32 between 0.0 and 1.0 (excluding 1.0)
    pub(crate) fn next_f32_unit(&mut self) -> f32 {
        randomize::f32_half_open_right(self.next_u32())
    }

    /// Random f32 between -1.0 and 1.0 (aka "Normalized Device Coordinates")
    pub(crate) fn next_f32_ndc(&mut self) -> f32 {
        randomize::f32_closed_neg_pos(self.next_u32())
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self(crate::control::Controls::default())
    }
}

impl Default for UpdateTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec3::default())
    }
}

impl Follow {
    pub(crate) fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            direction: Vec3::default(),
        }
    }
}
