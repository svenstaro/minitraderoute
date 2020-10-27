use raqote::*;
use shipyard::*;

use crate::components::*;

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
                (
                    Position {
                        x: position.0,
                        y: position.1,
                    },
                    Drawable {
                        width,
                        height,
                        image_data,
                    },
                ),
            );
        },
    );
}
