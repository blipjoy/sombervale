use crate::animation::{BlobAnims, FrogAnims, JeanAnims};
use crate::component::{Animation, Follow, Position, Sprite, Velocity};
use crate::image::{load_image, Image};
use ultraviolet::Vec2;

pub(crate) fn jean(x: f32, y: f32, z: f32) -> (Position, Velocity, Sprite, Animation<JeanAnims>) {
    let (width, height, image) = load_image(include_bytes!("../assets/jean.png"));

    let image = Image::new(image, Vec2::new(width as f32, height as f32));
    let pos = Position::new(x, y, z);
    let vel = Velocity::default();
    let sprite = Sprite {
        image,
        frame_height: 32,
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
    let (width, height, image) = load_image(include_bytes!("../assets/frog.png"));

    let image = Image::new(image, Vec2::new(width as f32, height as f32));
    let pos = Position::new(x, y, z);
    let vel = Velocity::default();
    let sprite = Sprite {
        image,
        frame_height: 19,
        frame_index: 0,
    };
    let anim = Animation(FrogAnims::new());

    (pos, vel, sprite, anim, follow)
}

pub(crate) fn blob(x: f32, y: f32, z: f32) -> (Position, Velocity, Sprite, Animation<BlobAnims>) {
    let (width, height, image) = load_image(include_bytes!("../assets/blob.png"));

    let image = Image::new(image, Vec2::new(width as f32, height as f32));
    let pos = Position::new(x, y, z);
    let vel = Velocity::default();
    let sprite = Sprite {
        image,
        frame_height: 25,
        frame_index: 0,
    };
    let anim = Animation(BlobAnims::new());

    (pos, vel, sprite, anim)
}
