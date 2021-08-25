use crate::animation::{FrogAnims, JeanAnims};
use crate::component::{Animation, CoordinateSpace, Follow, Position, Sprite, Velocity};

fn load_image(pcx: &[u8]) -> (usize, usize, Vec<u8>) {
    use std::io::Cursor;

    let mut pcx = pcx::Reader::new(Cursor::new(pcx)).unwrap();

    let width = pcx.width() as usize;
    let height = pcx.height() as usize;
    let stride = width * 3;

    let mut image = Vec::with_capacity(width as usize * height as usize * 3);

    for _ in 0..height {
        let mut row = Vec::with_capacity(stride);
        row.resize_with(stride, Default::default);
        pcx.next_row_rgb(&mut row).unwrap();

        image.extend(&row);
    }

    (width, height, image)
}

pub(crate) fn temp_bg() -> (Position, Sprite, CoordinateSpace) {
    let (width, height, image) = load_image(include_bytes!("../assets/temp_bg.pcx"));

    let pos = Position::new(0.0, 0.0, 0.0);
    let sprite = Sprite {
        width,
        height,
        image,
        frame_index: 0,
    };
    let space = CoordinateSpace::Screen;

    (pos, sprite, space)
}

pub(crate) fn jean(
    x: f32,
    y: f32,
    z: f32,
) -> (
    Position,
    Velocity,
    Sprite,
    CoordinateSpace,
    Animation<JeanAnims>,
) {
    let (width, _, image) = load_image(include_bytes!("../assets/jean.pcx"));

    let pos = Position::new(x, y, z);
    let vel = Velocity::default();
    let sprite = Sprite {
        width,
        height: 32,
        image,
        frame_index: 0,
    };
    let anim = Animation(JeanAnims::new());
    let space = CoordinateSpace::World;

    (pos, vel, sprite, space, anim)
}

pub(crate) fn frog(
    x: f32,
    y: f32,
    z: f32,
    follow: Follow,
) -> (
    Position,
    Velocity,
    Sprite,
    CoordinateSpace,
    Animation<FrogAnims>,
    Follow,
) {
    let (width, _, image) = load_image(include_bytes!("../assets/frog.pcx"));

    let pos = Position::new(x, y, z);
    let vel = Velocity::default();
    let sprite = Sprite {
        width,
        height: 19,
        image,
        frame_index: 0,
    };
    let anim = Animation(FrogAnims::new());
    let space = CoordinateSpace::World;

    (pos, vel, sprite, space, anim, follow)
}
