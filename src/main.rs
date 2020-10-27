use anyhow::{Context, Result};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoroshiro128StarStar;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const GAME_WIDTH: u32 = 100;
const GAME_HEIGTH: u32 = 100;

fn main() -> Result<()> {
    let mut rng = Xoroshiro128StarStar::seed_from_u64(1337);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Raqote")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(GAME_WIDTH, GAME_HEIGTH, surface_texture)?
    };

    let mut resize_count = 0;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if resize_count > 0 {
                let size = window.inner_size();
                pixels.resize(size.width, size.height);
                resize_count -= 1;
                dbg!(resize_count);
            }

            let frame = pixels.get_frame();
            for pixel in frame.chunks_exact_mut(4) {
                pixel[0] = rng.gen();
                pixel[1] = rng.gen();
                pixel[2] = rng.gen();
                pixel[3] = 0xff;
            }
            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                // Resize many times to work around this bug: https://github.com/parasyte/pixels/issues/121
                // Yes, it's stupid, but it works.
                resize_count = 100;
                pixels.resize(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}
