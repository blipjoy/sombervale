use crate::animation::{BlobAnims, BlobCurrentAnim, FrogAnims, JeanAnims};
use crate::component::{Animation, Follow, Position, Sprite, Velocity};
use crate::image::{load_image, Image};
use randomize::PCG32;
use std::collections::HashMap;
use tiled::PropertyValue;
use ultraviolet::{Vec2, Vec3};

type BlobStorage = (Position, Velocity, Sprite, Animation<BlobAnims>);

pub(crate) fn jean(pos: Vec3) -> (Position, Velocity, Sprite, Animation<JeanAnims>) {
    let (width, height, image) = load_image(include_bytes!("../assets/jean.png"));

    let image = Image::new(image, Vec2::new(width as f32, height as f32));
    let pos = Position(pos);
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
    pos: Vec3,
    follow: Follow,
) -> (Position, Velocity, Sprite, Animation<FrogAnims>, Follow) {
    let (width, height, image) = load_image(include_bytes!("../assets/frog.png"));

    let image = Image::new(image, Vec2::new(width as f32, height as f32));
    let pos = Position(pos);
    let vel = Velocity::default();
    let sprite = Sprite {
        image,
        frame_height: 19,
        frame_index: 0,
    };
    let anim = Animation(FrogAnims::new());

    (pos, vel, sprite, anim, follow)
}

pub(crate) fn blob(
    pos: Vec3,
    properties: &HashMap<String, PropertyValue>,
    random: &mut PCG32,
) -> BlobStorage {
    let (width, height, image) = load_image(include_bytes!("../assets/blob.png"));

    let image = Image::new(image, Vec2::new(width as f32, height as f32));
    let pos = Position(pos);
    let vel = Velocity::default();
    let sprite = Sprite {
        image,
        frame_height: 25,
        frame_index: 0,
    };
    let anim = Animation(BlobAnims::new(BlobCurrentAnim::new(
        random,
        properties.get("direction"),
    )));

    (pos, vel, sprite, anim)
}
