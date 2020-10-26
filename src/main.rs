use bevy::app::AppExit;
use bevy::prelude::*;

struct Planet;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(game_exit_system.system())
        .run();
}

fn game_exit_system(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: ResMut<Events<AppExit>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn setup() {}
