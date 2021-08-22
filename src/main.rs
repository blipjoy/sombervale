#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use randomize::PCG32;
use std::convert::TryInto;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 128;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let min_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let size = LogicalSize::new(WIDTH as f64 * 4.0, HEIGHT as f64 * 4.0);
        WindowBuilder::new()
            .with_title("Sombervale")
            .with_inner_size(size)
            .with_min_inner_size(min_size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let seed = generate_seed();
    let mut prng = PCG32::seed(seed.0, seed.1);

    // Populate the drawable world
    // TODO: May want to replace this with an ECS at some point
    let mut world: Vec<Box<dyn Drawable>> = vec![Box::new(ClearScreen::default())];
    for _ in 0..25 {
        let x = prng.next_u32() % WIDTH;
        let y = prng.next_u32() % HEIGHT;
        world.push(Box::new(SmallStar::new(x as usize, y as usize)));
    }
    for _ in 0..3 {
        let x = prng.next_u32() % WIDTH;
        let y = prng.next_u32() % HEIGHT;
        world.push(Box::new(BigStar::new(x as usize, y as usize)));
    }
    world.push(Box::new(Moon::new(83, 2)));
    world.push(Box::new(TempBG::default()));

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            for item in &world {
                item.draw(pixels.get_frame());
            }

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
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
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

/// Generate a pseudorandom seed for the game's PRNG.
fn generate_seed() -> (u64, u64) {
    use getrandom::getrandom;

    let mut seed = [0_u8; 16];

    getrandom(&mut seed).expect("failed to getrandom");

    (
        u64::from_ne_bytes(seed[0..8].try_into().unwrap()),
        u64::from_ne_bytes(seed[8..16].try_into().unwrap()),
    )
}

fn load_image(pcx: &[u8]) -> (usize, Vec<u8>) {
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

    (width, image)
}

struct ClearScreen {}

struct Moon {
    x: usize,
    y: usize,
    width: usize,
    image: Vec<u8>,
}

struct SmallStar {
    x: usize,
    y: usize,
    width: usize,
    image: Vec<u8>,
}

struct BigStar {
    x: usize,
    y: usize,
    width: usize,
    image: Vec<u8>,
}

struct TempBG {
    x: usize,
    y: usize,
    width: usize,
    image: Vec<u8>,
}

impl ClearScreen {
    fn new() -> Self {
        Self {}
    }
}

impl Moon {
    fn new(x: usize, y: usize) -> Self {
        let (width, image) = load_image(include_bytes!("../assets/moon.pcx"));

        Self { x, y, width, image }
    }
}

impl SmallStar {
    fn new(x: usize, y: usize) -> Self {
        let (width, image) = load_image(include_bytes!("../assets/small_star.pcx"));

        Self { x, y, width, image }
    }
}

impl BigStar {
    fn new(x: usize, y: usize) -> Self {
        let (width, image) = load_image(include_bytes!("../assets/big_star.pcx"));

        Self { x, y, width, image }
    }
}

impl TempBG {
    fn new() -> Self {
        let (width, image) = load_image(include_bytes!("../assets/temp_bg.pcx"));

        Self {
            x: 0,
            y: 16,
            width,
            image,
        }
    }
}

impl Default for ClearScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TempBG {
    fn default() -> Self {
        Self::new()
    }
}

trait Drawable {
    fn x(&self) -> usize {
        0
    }
    fn y(&self) -> usize {
        0
    }
    fn width(&self) -> usize {
        0
    }
    fn image(&self) -> &[u8] {
        &[]
    }

    fn draw(&self, pixels: &mut [u8]) {
        for (i, color) in self.image().chunks(3).enumerate() {
            if color != [0xff, 0, 0xff] {
                let x = self.x() + i % self.width();
                let y = self.y() + i / self.width();
                if x < WIDTH as usize && y < HEIGHT as usize {
                    let pos = (y * WIDTH as usize + x) * 4;
                    pixels[pos..pos + 3].copy_from_slice(color);
                }
            }
        }
    }
}

impl Drawable for ClearScreen {
    fn draw(&self, pixels: &mut [u8]) {
        for pixel in pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x1a, 0x1c, 0x2c, 0xff]);
        }
    }
}

impl Drawable for Moon {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }

    fn width(&self) -> usize {
        self.width
    }

    fn image(&self) -> &[u8] {
        &self.image
    }
}

impl Drawable for SmallStar {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }

    fn width(&self) -> usize {
        self.width
    }

    fn image(&self) -> &[u8] {
        &self.image
    }
}

impl Drawable for BigStar {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }

    fn width(&self) -> usize {
        self.width
    }

    fn image(&self) -> &[u8] {
        &self.image
    }
}

impl Drawable for TempBG {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }

    fn width(&self) -> usize {
        self.width
    }

    fn image(&self) -> &[u8] {
        &self.image
    }
}
