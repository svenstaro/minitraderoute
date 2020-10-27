use std::{sync::mpsc::channel, thread};

use raqote::*;

use anyhow::{Context, Result};

use rayon::prelude::*;

use audio::AudioEvent;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use shipyard::*;

use rand::{Rng, RngCore};
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoroshiro128StarStar;

mod audio;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const GAME_WIDTH: u32 = 100;
const GAME_HEIGTH: u32 = 100;

struct Position {
    x: u32,
    y: u32,
}

struct Drawable {
    width: u32,
    height: u32,
    image_data: Vec<u32>,
}

fn add_planet(world: &mut World, position: (u32, u32)) {
    let width = 20u32;
    let height = 20u32;

    let mut dt = DrawTarget::new(width as i32, height as i32);
    let mut pb = PathBuilder::new();
    pb.arc(160., 190., 180., 0., 2. * 3.14159);
    pb.close();
    let path = pb.finish();
    dt.push_clip(&path);
    let image_data = dt.into_vec();

    world.run(
        |mut entities: EntitiesViewMut,
         mut positions: ViewMut<Position>,
         mut drawables: ViewMut<Drawable>| {
            entities.add_entity(
                (&mut positions, &mut drawables),
                (Position { x: position.0, y: position.1 }, Drawable { width, height, image_data }),
            );
        },
    );
}

fn setup_world() -> World {
    let mut world = World::new();

    add_planet(&mut world, (30, 30));

    world
}

fn draw_system(frame: &mut [u8], positions: View<Position>, drawables: View<Drawable>) {
    frame
        .par_chunks_exact_mut(4)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, pixel)| {
            let x = (i % GAME_WIDTH as usize) as i16;
            let y = (i / GAME_WIDTH as usize) as i16;

            // TODO Actually draw drawables here
            pixel[0] = 0x00;
            pixel[1] = 0x00;
            pixel[2] = 0x00;
            pixel[3] = 0xff;
        });
}

fn main() -> Result<()> {
    let mut rng = Xoroshiro128StarStar::seed_from_u64(1337);

    let world = setup_world();

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

    let (snd_send, snd_recv) = channel();

    thread::spawn(move || {
        audio::start(snd_recv);
    });

    // snd_send.send(AudioEvent::Bass).unwrap();

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

        if input.update(&event) {
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
