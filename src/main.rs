use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

enum ResourceType {
    Minerals,
}

struct Planet;
struct Ship;
struct Station;
struct Resource(ResourceType);
struct Position(Vec2);

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(game_exit_system.system())
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

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    let red = materials.add(Color::rgb(0.8, 0.2, 0.2).into());

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(primitive(
                red,
                &mut meshes,
                ShapeType::Circle(60.0),
                TessellationMode::Fill(&FillOptions::default()),
                Vec3::new(0.0, 0.0, 0.0).into(),
        ))
        .spawn((Planet, Position(Vec2::new(50.0, 50.0))));
}
