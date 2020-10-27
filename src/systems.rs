use rayon::prelude::*;
use shipyard::*;

use crate::components::*;
use crate::GAME_WIDTH;

pub fn draw_system(frame: &mut [u8], _positions: View<Position>, _drawables: View<Drawable>) {
    frame
        .par_chunks_exact_mut(4)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, pixel)| {
            let _x = (i % GAME_WIDTH as usize) as i16;
            let _y = (i / GAME_WIDTH as usize) as i16;

            // TODO Actually draw drawables here
            pixel[0] = 0x00;
            pixel[1] = 0x00;
            pixel[2] = 0x00;
            pixel[3] = 0xff;
        });
}
