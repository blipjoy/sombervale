use crate::animation::{FrogAnims, FrogCurrentAnim, JeanAnims, JeanCurrentAnim};
use crate::component::{Animation, Controls, Follow, Position, Sprite, UpdateTime};
use crate::control::{Direction, Walk};
use crate::{HEIGHT, WIDTH};
use pixels::Pixels;
use shipyard::{IntoFastIter, UniqueView, UniqueViewMut, View, ViewMut, Workload, World};
use std::time::Instant;

pub(crate) fn register_systems(world: &World) {
    Workload::builder("draw")
        .with_system(&draw)
        .add_to_world(world)
        .expect("Register systems");

    Workload::builder("update")
        .with_system(&update_player)
        .with_system(&update_frog)
        .with_system(&update_time)
        .add_to_world(world)
        .expect("Register systems");
}

fn draw(mut pixels: UniqueViewMut<Pixels>, pos: View<Position>, sprite: View<Sprite>) {
    // Clear screen
    let frame = pixels.get_frame();
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0x1a, 0x1c, 0x2c, 0xff]);
    }

    (&pos, &sprite).fast_iter().for_each(|(pos, sprite)| {
        // Select the current frame
        let size = sprite.width * 3 * sprite.height;
        let start = sprite.frame_index * size;
        let end = start + size;
        let image = &sprite.image[start..end];

        // Draw the frame at the correct position
        for (i, color) in image.chunks(3).enumerate() {
            if color != [0xff, 0, 0xff] {
                let x = i % sprite.width + pos.0.x.round() as usize;
                let y = i / sprite.width + pos.0.z.round() as usize;

                if x < WIDTH as usize && y < HEIGHT as usize {
                    let pos = (y * WIDTH as usize + x) * 4;
                    frame[pos..pos + 3].copy_from_slice(color);
                }
            }
        }
    });
}

fn update_player(
    mut pos: ViewMut<Position>,
    mut anim: ViewMut<Animation<JeanCurrentAnim, JeanAnims>>,
    mut sprite: ViewMut<Sprite>,
    ut: UniqueView<UpdateTime>,
    controls: UniqueView<Controls>,
) {
    // Move 1 pixel every 16.667 ms.
    let dt = ut.0.elapsed();
    let velocity = dt.as_secs_f32() / 0.016667;

    // TODO: Angular velocity with sin() cos()

    (&mut pos, &mut anim, &mut sprite)
        .fast_iter()
        .for_each(|(pos, anim, sprite)| {
            // Update animation
            match controls.0.walk() {
                Walk::Walk(dir) => {
                    if dir & Direction::LEFT == Direction::LEFT {
                        anim.playing = JeanCurrentAnim::WalkLeft;
                        pos.0.x -= velocity;
                    }

                    if dir & Direction::RIGHT == Direction::RIGHT {
                        anim.playing = JeanCurrentAnim::WalkRight;
                        pos.0.x += velocity;
                    }

                    if dir & Direction::UP == Direction::UP {
                        anim.playing = JeanCurrentAnim::WalkRight; // FIXME: Need new animations
                        pos.0.z -= velocity;
                    }

                    if dir & Direction::DOWN == Direction::DOWN {
                        anim.playing = JeanCurrentAnim::WalkLeft; // FIXME: Need new animations
                        pos.0.z += velocity;
                    }
                }
                Walk::NoInput => {
                    anim.playing = match anim.playing {
                        JeanCurrentAnim::IdleLeft => JeanCurrentAnim::IdleLeft,
                        JeanCurrentAnim::IdleRight => JeanCurrentAnim::IdleRight,
                        JeanCurrentAnim::WalkLeft => JeanCurrentAnim::IdleLeft,
                        JeanCurrentAnim::WalkRight => JeanCurrentAnim::IdleRight,
                    };
                }
            }

            // Play animation
            let animation = match anim.playing {
                JeanCurrentAnim::IdleRight => &mut anim.animations.idle_right,
                JeanCurrentAnim::IdleLeft => &mut anim.animations.idle_left,
                JeanCurrentAnim::WalkRight => &mut anim.animations.walk_right,
                JeanCurrentAnim::WalkLeft => &mut anim.animations.walk_left,
            };

            let dur = animation.get_frame().duration;
            if let Some(index) = animation.update(dur) {
                sprite.frame_index = index;
            }
        });
}

fn update_frog(
    mut pos: ViewMut<Position>,
    mut anim: ViewMut<Animation<FrogCurrentAnim, FrogAnims>>,
    mut sprite: ViewMut<Sprite>,
    follow: View<Follow>,
    ut: UniqueView<UpdateTime>,
) {
    // Move 1 pixel every 9 ms.
    let dt = ut.0.elapsed();
    let velocity = dt.as_secs_f32() / 0.009;

    (&mut pos, &mut anim, &mut sprite, &follow)
        .fast_iter()
        .for_each(|(pos, anim, sprite, follow)| {
            let animation = match anim.playing {
                FrogCurrentAnim::IdleRight => &anim.animations.idle_right,
                FrogCurrentAnim::IdleLeft => &anim.animations.idle_left,
                FrogCurrentAnim::HopRight => &anim.animations.hop_right,
                FrogCurrentAnim::HopLeft => &anim.animations.hop_left,
            };
            let frame = animation.get_frame();

            match anim.playing {
                FrogCurrentAnim::HopRight => {
                    if frame.index != 0 && frame.index != 4 {
                        pos.0.x += velocity;
                    }

                    // DEBUG
                    if pos.0.x as usize >= crate::WIDTH as usize - sprite.width {
                        anim.playing = FrogCurrentAnim::HopLeft;
                        anim.animations.hop_left.current_index = 4;
                        anim.animations.hop_left.start_time = Instant::now();
                    }
                }
                FrogCurrentAnim::HopLeft => {
                    if frame.index != 5 && frame.index != 9 {
                        pos.0.x -= velocity;
                    }

                    // DEBUG
                    if pos.0.x <= 2.0 {
                        anim.playing = FrogCurrentAnim::HopRight;
                        anim.animations.hop_right.current_index = 9;
                        anim.animations.hop_right.start_time = Instant::now();
                    }
                }
                _ => {}
            }

            // Play animation
            let animation = match anim.playing {
                FrogCurrentAnim::IdleRight => &mut anim.animations.idle_right,
                FrogCurrentAnim::IdleLeft => &mut anim.animations.idle_left,
                FrogCurrentAnim::HopRight => &mut anim.animations.hop_right,
                FrogCurrentAnim::HopLeft => &mut anim.animations.hop_left,
            };
            let dur = animation.frames[animation.current_index % animation.frames.len()].duration;
            if let Some(index) = animation.update(dur) {
                sprite.frame_index = index;
            }
        });
}

fn update_time(mut dt: UniqueViewMut<UpdateTime>) {
    dt.0 = Instant::now();
}
