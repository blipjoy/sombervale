#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::component::{Controls, Hud, UpdateTime};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use shipyard::{UniqueViewMut, World};
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod animation;
mod component;
mod control;
mod entity;
mod power;
mod system;

pub(crate) const WIDTH: u32 = 160;
pub(crate) const HEIGHT: u32 = 128;

fn main() -> Result<(), Error> {
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

    // Populate the drawable world
    let mut world = World::default();

    world.add_unique(pixels).expect("Add pixels to world");
    world
        .add_unique(UpdateTime::default())
        .expect("Update time");
    world.add_unique(Controls::default()).expect("Controls");

    let hud = Hud {
        frog_power: Some(power::FrogPower::default()),
        ..Hud::default()
    };
    world.add_unique(hud).expect("HUD");

    world.add_entity(entity::temp_bg());
    world.add_entity(entity::jean(60.0, 0.0, 85.0));

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
        if let Event::DeviceEvent {
            event: DeviceEvent::Key(key),
            ..
        } = event
        {
            let mut controls = world
                .borrow::<UniqueViewMut<Controls>>()
                .expect("get pixels");

            controls.0.update(key);
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