use crate::animation::{Animated, FrogAnims, JeanAnims};
use crate::component::{
    Animation, Controls, CoordinateSpace, Follow, Hud, Position, Random, Sprite, UpdateTime,
    Velocity,
};
use crate::control::{Direction, Power, Walk};
use crate::{HEIGHT, WIDTH};
use pixels::Pixels;
use shipyard::{
    EntitiesViewMut, Get, IntoFastIter, IntoWithId, UniqueView, UniqueViewMut, View, ViewMut,
    Workload, World,
};
use std::f32::consts::TAU;
use std::time::Instant;
use ultraviolet::{Rotor3, Vec2, Vec3};

// Speeds are in pixels per second
const JEAN_SPEED: f32 = 60.0;
const FROG_SPEED: f32 = 180.0;

// Max distance where Frog will begin hopping toward Jean
const FROG_THRESHOLD: f32 = 28.0;

type FrogStorage<'a> = (
    ViewMut<'a, Position>,
    ViewMut<'a, Velocity>,
    ViewMut<'a, Sprite>,
    ViewMut<'a, CoordinateSpace>,
    ViewMut<'a, Animation<FrogAnims>>,
    ViewMut<'a, Follow>,
);

pub(crate) fn register_systems(world: &World) {
    Workload::builder("draw")
        .with_system(&draw)
        .with_system(&draw_hud)
        .add_to_world(world)
        .expect("Register systems");

    // TODO: Add system for summoning frogs
    Workload::builder("update")
        .with_system(&summon_frog)
        .with_system(&update_jean_velocity)
        .with_system(&update_frog_velocity)
        .with_system(&update_positions)
        .with_system(&update_animation::<JeanAnims>)
        .with_system(&update_animation::<FrogAnims>)
        .with_system(&update_hud)
        .with_system(&update_time)
        .add_to_world(world)
        .expect("Register systems");
}

fn draw(
    mut pixels: UniqueViewMut<Pixels>,
    positions: View<Position>,
    sprites: View<Sprite>,
    spaces: View<CoordinateSpace>,
) {
    // Clear screen
    let frame = pixels.get_frame();
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0x1a, 0x1c, 0x2c, 0xff]);
    }

    // Sort entities by Z coordinate
    let mut entities = (&positions, &sprites, &spaces)
        .fast_iter()
        .collect::<Vec<_>>();
    entities.sort_unstable_by_key(|(pos, _, &space)| match space {
        CoordinateSpace::World => -pos.0.z.round() as i32,
        CoordinateSpace::Screen => i32::MIN + (pos.0.z.round() as i32),
    });

    for (pos, sprite, &space) in entities {
        // Convert entity position to screen space
        let size = Vec2::new(sprite.width as f32, sprite.height as f32);
        let screen_pos = if space == CoordinateSpace::World {
            world_to_screen(pos.0, size)
        } else {
            pos.0.truncated()
        };

        // Select the current frame
        let size = sprite.width * 3 * sprite.height;
        let start = sprite.frame_index * size;
        let end = start + size;
        let image = &sprite.image[start..end];

        // Draw the frame at the correct position
        for (i, color) in image.chunks(3).enumerate() {
            if color != [0xff, 0, 0xff] {
                // FIXME: Allow negative positions
                let x = i % sprite.width + screen_pos.x.round() as usize;
                let y = i / sprite.width + screen_pos.y.round() as usize;

                if x < WIDTH as usize && y < HEIGHT as usize {
                    let index = (y * WIDTH as usize + x) * 4;
                    frame[index..index + 3].copy_from_slice(color);
                }
            }
        }

        // // DEBUG DRAWING
        // {
        //     let pos = world_to_screen(pos.0, Vec2::default());
        //     let x = pos.x.round() as usize;
        //     let y = pos.y.round() as usize;
        //     let width = WIDTH as usize;
        //     let height = HEIGHT as usize;
        //     if x < width && y >= 1 && y < height + 1 {
        //         let index = ((y - 1) * width + x) * 4;
        //         frame[index..index + 3].copy_from_slice(&[0xff, 0, 0xff]);
        //     }
        // }
    }
}

/// Convert world coordinates to screen coordinates.
fn world_to_screen(pos: Vec3, size: Vec2) -> Vec2 {
    let x = pos.x - size.x / 2.0;
    let y = HEIGHT as f32 - (pos.z + size.y);

    Vec2::new(x, y)
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

fn summon_frog(
    mut entities: EntitiesViewMut,
    mut controls: UniqueViewMut<Controls>,
    mut hud: UniqueViewMut<Hud>,
    mut random: UniqueViewMut<Random>,
    tag: View<Animation<JeanAnims>>,
    storage: FrogStorage,
) {
    // Get Jean's position
    let (pos, jean_id) = (&storage.0, &tag)
        .fast_iter()
        .with_id()
        .next()
        .map(|(id, (pos, _))| (pos.0, id))
        .expect("Where's Jean?!");

    // TODO: Select the correct power based on HUD
    if let Some(frog_power) = hud.frog_power.as_mut() {
        if controls.0.power() == Power::Use && frog_power.use_power() {
            let angle = random.next_f32() * TAU;
            let frog = crate::entity::frog(
                pos.x + angle.cos() * random.next_f32() * FROG_THRESHOLD,
                pos.y,
                pos.z + angle.sin() * random.next_f32() * FROG_THRESHOLD,
                Follow::new(jean_id),
            );

            entities.add_entity(storage, frog);
        }
    }
}

fn update_jean_velocity(
    mut velocities: ViewMut<Velocity>,
    mut animations: ViewMut<Animation<JeanAnims>>,
    controls: UniqueView<Controls>,
    ut: UniqueView<UpdateTime>,
) {
    use crate::animation::JeanCurrentAnim::*;

    let dt = ut.0.elapsed();
    let magnitude = Vec3::new(dt.as_secs_f32() / (1.0 / JEAN_SPEED), 0.0, 0.0);
    let entities = (&mut velocities, &mut animations).fast_iter();

    for (vel, anim) in entities {
        let (animation, angle) = match controls.0.walk() {
            Walk::Walk(Direction::RIGHT) => (WalkRight, TAU * (0.0 / 8.0)),
            Walk::Walk(Direction::UP_RIGHT) => (WalkRight, TAU * (1.0 / 8.0)),
            Walk::Walk(Direction::UP) => (anim.0.to_walking(), TAU * (2.0 / 8.0)),
            Walk::Walk(Direction::UP_LEFT) => (WalkLeft, TAU * (3.0 / 8.0)),
            Walk::Walk(Direction::LEFT) => (WalkLeft, TAU * (4.0 / 8.0)),
            Walk::Walk(Direction::DOWN_LEFT) => (WalkLeft, TAU * (5.0 / 8.0)),
            Walk::Walk(Direction::DOWN) => (anim.0.to_walking(), TAU * (6.0 / 8.0)),
            Walk::Walk(Direction::DOWN_RIGHT) => (WalkRight, TAU * (7.0 / 8.0)),
            _ => (anim.0.to_idle(), -1.0),
        };

        if anim.0.playing() != animation {
            anim.0.set(animation);
        }

        if angle >= 0.0 {
            let rotor = Rotor3::from_rotation_xz(angle);
            vel.0 = magnitude.rotated_by(rotor);
        } else {
            // TODO: Friction
            vel.0 = Vec3::default();
        }
    }
}

fn update_frog_velocity(
    mut velocities: ViewMut<Velocity>,
    mut animations: ViewMut<Animation<FrogAnims>>,
    mut following: ViewMut<Follow>,
    positions: View<Position>,
    mut random: UniqueViewMut<Random>,
    ut: UniqueView<UpdateTime>,
) {
    use crate::animation::FrogCurrentAnim::*;

    let dt = ut.0.elapsed();
    let magnitude = dt.as_secs_f32() / (1.0 / FROG_SPEED);
    let entities = (&mut velocities, &mut animations, &mut following, &positions).fast_iter();

    for (vel, anim, follow, pos) in entities {
        // Get Jean's position
        let jean_pos = positions.get(follow.entity_id).expect("Where's Jean?!").0;

        // Position of Jean relative to Frog
        let relative_pos = jean_pos - pos.0;

        let frame_index = anim.0.get_frame_index();

        // Update the direction only when the Frog is idling
        // AND it is far away from the target
        if relative_pos.mag() > FROG_THRESHOLD
            && (anim.0.playing() == IdleLeft || anim.0.playing() == IdleRight)
        {
            let animation = if relative_pos.x > 0.0 {
                HopRight
            } else {
                HopLeft
            };
            anim.0.set(animation);
            follow.direction = relative_pos.normalized() * (random.next_f32() * 0.3 + 0.7);
        }

        // Frog ONLY moves when the animation frame is hopping
        vel.0 = if frame_index != 0 && frame_index != 4 && frame_index != 5 && frame_index != 9 {
            follow.direction * magnitude
        } else {
            Vec3::default()
        };
    }
}

fn update_animation<A: Animated + 'static>(
    mut animations: ViewMut<Animation<A>>,
    mut sprite: ViewMut<Sprite>,
) {
    let entities = (&mut animations, &mut sprite).fast_iter();

    for (anim, sprite) in entities {
        sprite.frame_index = anim.0.animate();
    }
}

fn update_positions(mut positions: ViewMut<Position>, velocities: View<Velocity>) {
    let entities = (&mut positions, &velocities).fast_iter();

    for (pos, vel) in entities {
        // TODO: Collision detection?
        pos.0 += vel.0;
    }
}

fn update_hud(mut hud: UniqueViewMut<Hud>, frogs: View<Animation<FrogAnims>>) {
    if let Some(frog_power) = &mut hud.frog_power {
        frog_power.update(frogs.len());
    }
}

fn update_time(mut dt: UniqueViewMut<UpdateTime>) {
    dt.0 = Instant::now();
}
