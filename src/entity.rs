use crate::animation::{BlobAnims, FrogAnims, JeanAnims};
use crate::component::{Animation, Follow, Position, Sprite, Velocity};
use crate::image::load_image;

pub(crate) fn jean(x: f32, y: f32, z: f32) -> (Position, Velocity, Sprite, Animation<JeanAnims>) {
    let (width, _, image) = load_image(include_bytes!("../assets/jean.png"));

    let pos = Position::new(x, y, z);
    let vel = Velocity::default();
    let sprite = Sprite {
        width,
        height: 32,
        image,
        frame_index: 0,
    };
    let anim = Animation(JeanAnims::new());

    (pos, vel, sprite, anim)
}

pub(crate) fn frog(
    x: f32,
    y: f32,
    z: f32,
    follow: Follow,
) -> (Position, Velocity, Sprite, Animation<FrogAnims>, Follow) {
    let (width, _, image) = load_image(include_bytes!("../assets/frog.png"));

    let pos = Position::new(x, y, z);
    let vel = Velocity::default();
    let sprite = Sprite {
        width,
        height: 19,
        image,
        frame_index: 0,
    };
    let anim = Animation(FrogAnims::new());

    (pos, vel, sprite, anim, follow)
}

pub(crate) fn blob(x: f32, y: f32, z: f32) -> (Position, Velocity, Sprite, Animation<BlobAnims>) {
    let (width, _, image) = load_image(include_bytes!("../assets/blob.png"));

    let pos = Position::new(x, y, z);
    let vel = Velocity::default();
    let sprite = Sprite {
        width,
        height: 25,
        image,
        frame_index: 0,
    };
    let anim = Animation(BlobAnims::new());

    (pos, vel, sprite, anim)
}
