use crate::animation::{FrogAnims, FrogCurrentAnim, JeanAnims, JeanCurrentAnim};
use crate::component::{Animation, Controls, Follow, Hud, Position, Sprite, UpdateTime};
use crate::control::{Direction, Power, Walk};
use crate::entity;
use crate::{HEIGHT, WIDTH};
use pixels::Pixels;
use shipyard::{
    EntitiesViewMut, IntoFastIter, IntoWithId, UniqueView, UniqueViewMut, View, ViewMut, Workload,
    World,
};
use std::time::Instant;

pub(crate) fn register_systems(world: &World) {
    Workload::builder("draw")
        .with_system(&draw)
        .with_system(&draw_hud)
        .add_to_world(world)
        .expect("Register systems");

    Workload::builder("update")
        .with_system(&update_player)
        .with_system(&update_frog)
        .with_system(&update_hud)
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
        // // Debug position
        // let index = (pos.0.z.round() as usize * WIDTH as usize + pos.0.x.round() as usize) * 4;
        // frame[index..index + 3].copy_from_slice(&[0xff, 0, 0xff]);

        // Select the current frame
        let size = sprite.width * 3 * sprite.height;
        let start = sprite.frame_index * size;
        let end = start + size;
        let image = &sprite.image[start..end];

        // Draw the frame at the correct position
        for (i, color) in image.chunks(3).enumerate() {
            if color != [0xff, 0, 0xff] {
                // TODO: Treat position as lower center of sprite, not upper left corner
                let x = i % sprite.width + pos.0.x.round() as usize;
                let y = i / sprite.width + pos.0.z.round() as usize;

                if x < WIDTH as usize && y < HEIGHT as usize {
                    let index = (y * WIDTH as usize + x) * 4;
                    frame[index..index + 3].copy_from_slice(color);
                }
            }
        }
    });
}

fn draw_hud(mut pixels: UniqueViewMut<Pixels>, hud: UniqueView<Hud>) {
    let frame = pixels.get_frame();

    // FIXME: Draw Frog Power HUD
    if let Some(frog_power) = &hud.frog_power {
        let green = &[0x38, 0xb7, 0x64];
        let gray = &[0x94, 0xb0, 0xc2];

        // Draw a naive meter
        for y in 5..8 {
            for x in 5..25 {
                // Compute the HUD bar width
                let ratio = frog_power.level() as f32 / frog_power.max_level() as f32;
                let color = if (x - 5) as f32 / 20.0 < ratio {
                    green
                } else {
                    gray
                };

                let index = (y * WIDTH as usize + x) * 4;
                frame[index..index + 3].copy_from_slice(color);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn update_player(
    mut pos: ViewMut<Position>,
    mut anim: ViewMut<Animation<JeanCurrentAnim, JeanAnims>>,
    mut sprite: ViewMut<Sprite>,
    mut follow: ViewMut<Follow>,
    mut frog_anims: ViewMut<Animation<FrogCurrentAnim, FrogAnims>>,
    ut: UniqueView<UpdateTime>,
    mut controls: UniqueViewMut<Controls>,
    mut hud: UniqueViewMut<Hud>,
    mut entities: EntitiesViewMut,
) {
    use JeanCurrentAnim::*;

    // Move 1 pixel every 16.667 ms.
    let dt = ut.0.elapsed();
    let velocity = dt.as_secs_f32() / 0.016667;

    // TODO: Angular velocity with sin() cos()

    let mut new_frogs = Vec::new();

    (&mut pos, &mut anim, &mut sprite)
        .fast_iter()
        .with_id()
        .for_each(|(id, (pos, anim, sprite))| {
            // Update animation
            match controls.0.walk() {
                Walk::Walk(dir) => {
                    if dir & Direction::UP == Direction::UP {
                        anim.playing = WalkRight; // FIXME: Need new animations
                        pos.0.z -= velocity;
                    }

                    if dir & Direction::DOWN == Direction::DOWN {
                        anim.playing = WalkLeft; // FIXME: Need new animations
                        pos.0.z += velocity;
                    }

                    if dir & Direction::LEFT == Direction::LEFT {
                        anim.playing = WalkLeft;
                        pos.0.x -= velocity;
                    }

                    if dir & Direction::RIGHT == Direction::RIGHT {
                        anim.playing = WalkRight;
                        pos.0.x += velocity;
                    }
                }
                Walk::NoInput => {
                    anim.playing = match anim.playing {
                        IdleLeft | WalkLeft => IdleLeft,
                        IdleRight | WalkRight => IdleRight,
                    };
                }
            }

            if controls.0.power() == Power::Use && hud.frog_power.is_some() {
                // TODO: Choose the correct power, and execute appropriately.
                if hud.frog_power.as_mut().unwrap().use_power() {
                    new_frogs.push(entity::frog(
                        pos.0.x + 10.0, // FIXME: Randomize!
                        pos.0.y,
                        pos.0.z + 10.0, // FIXME: Randomize!
                        Follow(id),
                    ));
                }
            }

            // Play animation
            let animation = match anim.playing {
                IdleRight => &mut anim.animations.idle_right,
                IdleLeft => &mut anim.animations.idle_left,
                WalkRight => &mut anim.animations.walk_right,
                WalkLeft => &mut anim.animations.walk_left,
            };

            let dur = animation.get_frame().duration;
            if let Some(index) = animation.update(dur) {
                sprite.frame_index = index;
            }
        });

    // Add new frogs
    if !new_frogs.is_empty() {
        entities.bulk_add_entity(
            (&mut pos, &mut sprite, &mut frog_anims, &mut follow),
            new_frogs,
        );
    }
}

fn update_frog(
    mut pos: ViewMut<Position>,
    mut anim: ViewMut<Animation<FrogCurrentAnim, FrogAnims>>,
    mut sprite: ViewMut<Sprite>,
    // follow: View<Follow>,
    jean: View<Animation<JeanCurrentAnim, JeanAnims>>,
    ut: UniqueView<UpdateTime>,
) {
    use FrogCurrentAnim::*;

    // Move 1 pixel every 9 ms.
    let dt = ut.0.elapsed();
    let velocity = dt.as_secs_f32() / 0.009;

    // FIXME: Try this instead: https://leudz.github.io/shipyard/guide/0.5.0/fundamentals/get-and-modify.html
    // Currently does not work because the entity ID is not static.
    let jean_pos = (&pos, &jean)
        .fast_iter()
        .next()
        .map(|(pos, _)| pos.0)
        .expect("Where's Jean?!");

    (&mut pos, &mut anim, &mut sprite)
        .fast_iter()
        .for_each(|(pos, anim, sprite)| {
            let animation = match anim.playing {
                IdleRight => &anim.animations.idle_right,
                IdleLeft => &anim.animations.idle_left,
                HopRight => &anim.animations.hop_right,
                HopLeft => &anim.animations.hop_left,
            };
            let frame = animation.get_frame();

            // Distance where Frog will stop moving toward Jean
            const THRESHOLD: f32 = 32.0;

            enum Dir {
                NorthSouth,
                WestEast,
            }

            // Follow Jean by setting the animation
            // TODO: Fix this so hard! Movement should be based on the direction to jean_pos
            let mut chasing = None;
            if pos.0.x >= jean_pos.x + THRESHOLD {
                chasing = Some(Dir::WestEast);
                anim.playing = HopLeft;
            } else if pos.0.x <= jean_pos.x - THRESHOLD {
                chasing = Some(Dir::WestEast);
                anim.playing = HopRight;
            }

            if pos.0.z >= jean_pos.z + THRESHOLD {
                chasing = Some(Dir::NorthSouth);
                anim.playing = HopLeft;
            } else if pos.0.z <= jean_pos.z - THRESHOLD {
                chasing = Some(Dir::NorthSouth);
                anim.playing = HopRight;
            }

            // Make frog stop moving when it touches the ground
            if frame.index != 0 && frame.index != 4 && frame.index != 5 && frame.index != 9 {
                match anim.playing {
                    HopLeft => {
                        if frame.index != 5 && frame.index != 9 {
                            match chasing {
                                Some(Dir::NorthSouth) => {
                                    pos.0.z -= velocity;
                                }
                                _ => {
                                    pos.0.x -= velocity;
                                }
                            }
                        }
                    }
                    HopRight => {
                        if frame.index != 0 && frame.index != 4 {
                            match chasing {
                                Some(Dir::NorthSouth) => {
                                    pos.0.z += velocity;
                                }
                                _ => {
                                    pos.0.x += velocity;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            } else if chasing.is_none() {
                anim.playing = match anim.playing {
                    IdleRight | HopRight => IdleRight,
                    IdleLeft | HopLeft => IdleLeft,
                }
            }

            // Play animation
            let animation = match anim.playing {
                IdleRight => &mut anim.animations.idle_right,
                IdleLeft => &mut anim.animations.idle_left,
                HopRight => &mut anim.animations.hop_right,
                HopLeft => &mut anim.animations.hop_left,
            };
            let dur = animation.frames[animation.current_index % animation.frames.len()].duration;
            if let Some(index) = animation.update(dur) {
                sprite.frame_index = index;
            }
        });
}

fn update_hud(mut hud: UniqueViewMut<Hud>, frogs: View<Animation<FrogCurrentAnim, FrogAnims>>) {
    if let Some(frog_power) = &mut hud.frog_power {
        frog_power.update(frogs.len());
    }
}

fn update_time(mut dt: UniqueViewMut<UpdateTime>) {
    dt.0 = Instant::now();
}
