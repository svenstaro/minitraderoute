use raqote::*;
use rayon::prelude::*;
use shipyard::*;

use crate::components::*;
use crate::{GAME_HEIGTH, GAME_WIDTH};

pub fn draw_system(
    draw_target: &mut DrawTarget,
    _positions: View<Position>,
    drawables: View<Drawable>,
) {
    for drawable in drawables.iter() {
        draw_target.push_clip(&drawable.path);
        draw_target.fill(
            &drawable.path,
            &SolidSource::from_unpremultiplied_argb(0, 255, 20, 20).into(),
            &DrawOptions::new(),
        )
    }
}
