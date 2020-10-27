use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

enum ResourceType {
    Minerals,
}

struct Planet;
struct Ship;
struct Station;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(game_exit_system.system())
        .add_system(ship_movement_system.system())
        .run();
}

fn game_exit_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn ship_movement_system(time: Res<Time>, mut ship_query: Query<(&Ship, &mut Transform)>) {
    let direction = Vec3::new(0.5, 0.5, 0.0).normalize();
    for (ship, mut transform) in &mut ship_query.iter() {
        transform.translate(100.0 * direction * time.delta_seconds);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let red = materials.add(Color::rgb(0.8, 0.2, 0.2).into());
    let blue = materials.add(Color::rgb(0.2, 0.2, 0.8).into());

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(primitive(
            blue,
            &mut meshes,
            ShapeType::RoundedRectangle {
                width: 100.0,
                height: 100.0,
                border_radius: 20.0,
            },
            TessellationMode::Fill(&FillOptions::default()),
            Vec3::new(200.0, 0.0, 0.0).into(),
        ))
        .with(Ship);
}
