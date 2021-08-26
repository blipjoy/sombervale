use crate::animation::{Animated, BlobAnims, FrogAnims, JeanAnims};
use crate::component::{
    Animation, Controls, Follow, Hud, Position, Random, Sprite, Tilemap, UpdateTime, Velocity,
    Viewport,
};
use crate::control::{Direction, Power, Walk};
use crate::image::{blit, ImageViewMut};
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
const BLOB_SPEED: f32 = 70.0;

// Max distance where Frog will begin hopping toward Jean
const FROG_THRESHOLD: f32 = 28.0;

const SCREEN_SIZE: Vec2 = Vec2::new(WIDTH as f32, HEIGHT as f32);
const BOUNDS_MIN: Vec2 = Vec2::new(32.0, 32.0);
const BOUNDS_MAX: Vec2 = Vec2::new(WIDTH as f32 - 32.0, HEIGHT as f32 - 32.0);

type FrogStorage<'a> = (
    ViewMut<'a, Position>,
    ViewMut<'a, Velocity>,
    ViewMut<'a, Sprite>,
    ViewMut<'a, Animation<FrogAnims>>,
    ViewMut<'a, Follow>,
);

pub(crate) fn register_systems(world: &World) {
    Workload::builder("draw")
        .with_system(&draw_tilemap)
        .with_system(&draw_sprite)
        .with_system(&draw_hud)
        .add_to_world(world)
        .expect("Register systems");

    // TODO: Add system for summoning frogs
    Workload::builder("update")
        .with_system(&summon_frog)
        .with_system(&update_jean_velocity)
        .with_system(&update_frog_velocity)
        .with_system(&update_blob_velocity)
        .with_system(&update_positions)
        .with_system(&update_viewport)
        .with_system(&update_animation::<JeanAnims>)
        .with_system(&update_animation::<FrogAnims>)
        .with_system(&update_animation::<BlobAnims>)
        .with_system(&update_hud)
        .with_system(&update_time)
        .add_to_world(world)
        .expect("Register systems");
}

/// Convert world coordinates to screen coordinates.
fn world_to_screen(pos: Vec3, size: Vec2, viewport: &Viewport) -> Vec2 {
    let x = pos.x - size.x / 2.0;
    let y = viewport.world_height - (pos.z + size.y);
    let pos = Vec2::new(x, y) - viewport.pos;

    Vec2::new(pos.x.floor(), pos.y.floor())
}

fn draw_tilemap(
    mut pixels: UniqueViewMut<Pixels>,
    viewport: UniqueView<Viewport>,
    tilemaps: View<Tilemap>,
) {
    // Clear screen
    let mut frame = pixels.get_frame();
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0x1a, 0x1c, 0x2c, 0xff]);
    }

    let mut dest = ImageViewMut::new(&mut frame, SCREEN_SIZE);
    let dest_pos = Vec2::default();

    for (layer,) in (&tilemaps,).fast_iter() {
        let src_pos = viewport.pos * layer.parallax;
        blit(&mut dest, dest_pos, &layer.image, src_pos, SCREEN_SIZE);
    }
}

fn draw_sprite(
    mut pixels: UniqueViewMut<Pixels>,
    viewport: UniqueView<Viewport>,
    positions: View<Position>,
    sprites: View<Sprite>,
) {
    let mut frame = pixels.get_frame();

    // Create a single ImageViewMut that is shared over all sprites when debug mode is disabled
    #[cfg(not(feature = "debug-mode"))]
    let mut dest = ImageViewMut::new(&mut frame, SCREEN_SIZE);

    // Sort entities by Z coordinate
    let mut entities = (&positions, &sprites).fast_iter().collect::<Vec<_>>();
    entities.sort_unstable_by_key(|(pos, _)| -pos.0.z as i32);

    for (pos, sprite) in entities {
        // Convert entity position to screen space
        let frame_size = Vec2::new(sprite.image.size().x, sprite.frame_height as f32);
        let dest_pos = world_to_screen(pos.0, frame_size, &viewport);

        // Select the current frame
        let src_pos = Vec2::new(0.0, sprite.frame_index as f32 * sprite.frame_height as f32);

        // DEBUG: We need a temporary ImageViewMut so that we can draw directly to the buffer later
        #[cfg(feature = "debug-mode")]
        let mut dest = ImageViewMut::new(&mut frame, SCREEN_SIZE);

        // Copy source image to destination frame
        blit(&mut dest, dest_pos, &sprite.image, src_pos, frame_size);

        // DEBUG DRAWING
        #[cfg(feature = "debug-mode")]
        {
            // Pink dot for upper left corner
            let screen_pos = world_to_screen(pos.0, frame_size, &viewport);
            let x = screen_pos.x as isize;
            let y = screen_pos.y as isize;
            let width = WIDTH as isize;
            let height = HEIGHT as isize;
            if x >= 0 && x < width && y >= 1 && y < height + 1 {
                let index = (((y - 1) * width + x) * 4) as usize;
                frame[index..index + 4].copy_from_slice(&[0xff, 0, 0xff, 0xff]);
            }

            // Red dot for feet ("world position")
            let screen_pos = world_to_screen(pos.0, frame_size, &viewport)
                + Vec2::new(frame_size.x / 2.0, frame_size.y);
            let x = screen_pos.x as isize;
            let y = screen_pos.y as isize;
            let width = WIDTH as isize;
            let height = HEIGHT as isize;
            if x >= 0 && x < width && y >= 1 && y < height + 1 {
                let index = (((y - 1) * width + x) * 4) as usize;
                frame[index..index + 4].copy_from_slice(&[0xff, 0, 0, 0xff]);
            }
        }
    }
}

fn draw_hud(
    mut pixels: UniqueViewMut<Pixels>,
    _viewport: UniqueView<Viewport>,
    hud: UniqueView<Hud>,
) {
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

fn update_blob_velocity(
    mut velocities: ViewMut<Velocity>,
    mut animations: ViewMut<Animation<BlobAnims>>,
    mut random: UniqueViewMut<Random>,
    ut: UniqueView<UpdateTime>,
) {
    use crate::animation::BlobCurrentAnim::*;

    let dt = ut.0.elapsed();
    let magnitude = dt.as_secs_f32() / (1.0 / BLOB_SPEED);
    let entities = (&mut velocities, &mut animations).fast_iter();

    for (vel, anim) in entities {
        // When not moving, randomly decide on a new direction to bounce
        if vel.0.mag() < 0.01 && random.next_f32() < 0.01 {
            let angle = random.next_f32() * TAU;
            let rotor = Rotor3::from_rotation_xz(angle);
            vel.0 = Vec3::unit_x().rotated_by(rotor) * magnitude;

            let animation = if vel.0.x > 0.0 {
                BounceRight
            } else {
                BounceLeft
            };
            anim.0.set(animation);
        }

        if let IdleLeft | IdleRight = anim.0.playing() {
            vel.0 = Vec3::default();
        }
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

fn update_viewport(
    mut viewport: UniqueViewMut<Viewport>,
    positions: View<Position>,
    sprites: View<Sprite>,
    tag: View<Animation<JeanAnims>>,
) {
    for (pos, sprite, _) in (&positions, &sprites, &tag).fast_iter() {
        let viewport_basis = Viewport {
            pos: Vec2::default(),
            world_height: viewport.world_height,
        };
        let size = Vec2::new(sprite.image.size().x, sprite.frame_height as f32);
        let sprite_min = world_to_screen(pos.0, size, &viewport_basis);
        let sprite_max = sprite_min + size;

        // FIXME: Use clamp instead of multiple conditions
        if sprite_max.x > (viewport.pos.x + BOUNDS_MAX.x) {
            viewport.pos.x = sprite_max.x - BOUNDS_MAX.x;
        }
        if sprite_max.y > (viewport.pos.y + BOUNDS_MAX.y) {
            viewport.pos.y = sprite_max.y - BOUNDS_MAX.y;
        }

        if sprite_min.x < (viewport.pos.x + BOUNDS_MIN.x) {
            viewport.pos.x = sprite_min.x - BOUNDS_MIN.x;
        }
        if sprite_min.y < (viewport.pos.y + BOUNDS_MIN.y) {
            viewport.pos.y = sprite_min.y - BOUNDS_MIN.y;
        }

        // TODO: Constrain viewport position to the world size
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
