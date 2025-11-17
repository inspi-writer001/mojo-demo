mod components;
mod resources;
mod states;
mod systems;
mod types;

use bevy::prelude::*;
use states::AppState;
use systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Mojo Game example".into(),
                        resolution: (640.0, 480.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (character_movement, update_health_bar, check_game_over)
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(OnEnter(AppState::GameOver), show_game_over)
        .add_systems(
            Update,
            handle_game_over_buttons.run_if(in_state(AppState::GameOver)),
        )
        .run();
}
