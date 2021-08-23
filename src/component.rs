use shipyard::EntityId;
use std::time::Instant;
use ultraviolet::vec::Vec3;

pub(crate) struct Controls(pub(crate) crate::control::Controls);

impl Default for Controls {
    fn default() -> Self {
        Self(crate::control::Controls::default())
    }
}

pub(crate) struct UpdateTime(pub(crate) Instant);

impl Default for UpdateTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}

pub(crate) struct Position(pub(crate) Vec3);

impl Position {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

pub(crate) struct Sprite {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) image: Vec<u8>,
    pub(crate) frame_index: usize,
}

pub(crate) struct Animation<P, A> {
    pub(crate) playing: P,
    pub(crate) animations: A,
}

pub(crate) struct Follow(pub(crate) EntityId);
