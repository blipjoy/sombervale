use crate::animation::{FrogAnims, FrogCurrentAnim, JeanAnims, JeanCurrentAnim};
use crate::component::{Animation, Follow, Position, Sprite};

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

pub(crate) fn temp_bg() -> (Position, Sprite) {
    let (width, height, image) = load_image(include_bytes!("../assets/temp_bg.pcx"));

    let pos = Position::new(0.0, 0.0, 0.0);
    let sprite = Sprite {
        width,
        height,
        image,
        frame_index: 0,
    };

    (pos, sprite)
}

pub(crate) fn jean(
    x: f32,
    y: f32,
    z: f32,
) -> (Position, Sprite, Animation<JeanCurrentAnim, JeanAnims>) {
    let (width, _, image) = load_image(include_bytes!("../assets/jean.pcx"));

    let pos = Position::new(x, y, z);
    let sprite = Sprite {
        width,
        height: 32,
        image,
        frame_index: 0,
    };
    let anim = Animation {
        playing: JeanCurrentAnim::IdleRight,
        animations: JeanAnims::new(),
    };

    (pos, sprite, anim)
}

pub(crate) fn frog(
    x: f32,
    y: f32,
    z: f32,
    follow: Follow,
) -> (
    Position,
    Sprite,
    Animation<FrogCurrentAnim, FrogAnims>,
    Follow,
) {
    let (width, _, image) = load_image(include_bytes!("../assets/frog.pcx"));

    let pos = Position::new(x, y, z);
    let sprite = Sprite {
        width,
        height: 19,
        image,
        frame_index: 0,
    };
    let anim = Animation {
        playing: FrogCurrentAnim::HopRight,
        animations: FrogAnims::new(),
    };

    (pos, sprite, anim, follow)
}
