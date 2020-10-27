use std::{sync::mpsc::channel, thread};

use anyhow::{Context, Result};
use audio::AudioEvent;
use pixels::{Pixels, SurfaceTexture};
use rand::{Rng, RngCore};
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoroshiro128StarStar;
use raqote::*;
use rayon::prelude::*;
use shipyard::*;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod audio;
mod components;
mod systems;
mod world;

use components::*;
use systems::*;
use world::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const GAME_WIDTH: u32 = 100;
const GAME_HEIGTH: u32 = 100;

fn main() -> Result<()> {
    //let mut rng = Xoroshiro128StarStar::seed_from_u64(1337);

    // Init audio channels and spawn the audio thread
    let (_snd_send, snd_recv) = channel();
    thread::spawn(move || {
        audio::start(snd_recv);
    });

    // Initialize pretty much everything that's important for our game.
    // Event loop: Catch redraw requests and user input
    // Window input helper
    // The actual window.
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
    // snd_send.send(AudioEvent::Bass).unwrap();

    // Initialize the shipyard world
    let world = setup_world();

    // Initialize our canvas
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(GAME_WIDTH, GAME_HEIGTH, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            world.run_with_data(draw_system, frame);

            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Check if there's some user input in this event.
        if input.update(&event) {
            // Exit program escape
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}
