use raqote::*;
use shipyard::*;

use crate::components::*;
use crate::{GAME_HEIGTH, GAME_WIDTH};

/// Create the shipyard world.
/// This also does basic world initialization and entity creation.
pub fn setup_world() -> World {
    let mut world = World::new();

    add_planet(&mut world, (30, 30));

    world
}

fn add_planet(world: &mut World, position: (u32, u32)) {
    let width = 20u32;
    let height = 20u32;

    let mut pb = PathBuilder::new();
    pb.arc(160., 190., 180., 0., 2. * 3.14159);
    pb.close();

    world.run(
        |mut entities: EntitiesViewMut,
         mut positions: ViewMut<Position>,
         mut drawables: ViewMut<Drawable>| {
            entities.add_entity(
                (&mut positions, &mut drawables),
                (
                    Position {
                        x: position.0,
                        y: position.1,
                    },
                    Drawable {
                        width,
                        height,
                        path: pb.finish(),
                    },
                ),
            );
        },
    );
}
