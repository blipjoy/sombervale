#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::component::{Audio, Controls};
use crate::world::load_world;
use anyhow::Result;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use shipyard::{AllStoragesViewMut, NonSync, UniqueViewMut, World};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod animation;
mod audio;
mod component;
mod control;
mod entity;
mod hud;
mod image;
mod map;
mod power;
mod system;
mod world;

pub(crate) const WIDTH: u32 = 160;
pub(crate) const HEIGHT: u32 = 128;

fn main() -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let min_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let size = LogicalSize::new(WIDTH as f64 * 6.0, HEIGHT as f64 * 6.0);
        WindowBuilder::new()
            .with_title("Sombervale")
            .with_inner_size(size)
            .with_min_inner_size(min_size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    // Populate the world
    let world = World::default();
    let storages = world.borrow::<AllStoragesViewMut>().unwrap();
    storages.add_unique(pixels);
    storages.add_unique_non_sync(Audio::new()?);
    load_world(storages);

    // TODO: Move this somewhere else?
    {
        let storages = world.borrow::<AllStoragesViewMut>().unwrap();
        let mut audio = storages
            .borrow::<NonSync<UniqueViewMut<Audio>>>()
            .expect("Needs audio");
        audio.0.music()?;
    }

    system::register_systems(&world);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.run_workload("draw").expect("draw workload");

            let mut pixels = world.borrow::<UniqueViewMut<Pixels>>().expect("get pixels");
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle controls
        if let Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } = event
        {
            let mut controls = world
                .borrow::<UniqueViewMut<Controls>>()
                .expect("get pixels");

            controls.0.update(input);
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                let mut pixels = world.borrow::<UniqueViewMut<Pixels>>().expect("get pixels");
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.run_workload("update").expect("update workload");
            window.request_redraw();
        }
    });
}
