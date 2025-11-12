use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bytemuck::{Pod, Zeroable};
use mojo_rust_sdk::{client::WorldClient, world::World};
use solana_keypair::{read_keypair_file, Keypair};

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
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

#[derive(Resource)]
struct GameState {
    keypair: Keypair,
    state_name: String,
    position: MyPosition,
    rpc_client: WorldClient,
    er_client: WorldClient,
    last_move_timer: Timer,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let payer = read_keypair_file("dev_wallet-keypair.json").expect("Couldn't find wallet file");

    let rpc_client = WorldClient::new("https://api.devnet.solana.com");
    let er_rpc_client = WorldClient::new("https://devnet-eu.magicblock.app/");

    let new_world = World::create_world(&rpc_client, &payer, "moving_game8");
    let state_name = "brother_position8";
    println!("we got world yayy, {}", new_world.unwrap());

    let player_position = MyPosition { x: 0.0, y: 0.0 };
    let new_player =
        World::create_state::<MyPosition>(&rpc_client, &payer, &state_name, &player_position);
    println!(
        "we just spawned a character position and delegated, {}",
        new_player.unwrap()
    );

    // Insert resources so they can be accessed in systems
    commands.insert_resource(GameState {
        keypair: payer,
        state_name: state_name.to_string(),
        position: player_position,
        rpc_client,
        er_client: er_rpc_client,
        last_move_timer: Timer::from_seconds(2.0, TimerMode::Once),
    });

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GREEN),
        },
        ..Default::default()
    });

    let texture = asset_server.load("character.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 100.0, y: 100.0 }),
            ..default()
        },
        texture,
        ..default()
    });
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
) {
    let mut has_moved = false;

    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::Up) {
            transform.translation.y += 100.0 * time.delta_seconds();
            game_state.position.y += 100.0 * time.delta_seconds();
            has_moved = true;
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= 100.0 * time.delta_seconds();
            game_state.position.y -= 100.0 * time.delta_seconds();
            has_moved = true;
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += 100.0 * time.delta_seconds();
            game_state.position.x += 100.0 * time.delta_seconds();
            has_moved = true;
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= 100.0 * time.delta_seconds();
            game_state.position.x -= 100.0 * time.delta_seconds();
            has_moved = true;
        }
    }

    // Reset timer if moved
    if has_moved {
        game_state.last_move_timer.reset();
    }

    // Tick the timer
    game_state.last_move_timer.tick(time.delta());

    // Send to MagicBlock after 2 seconds of inactivity
    if game_state.last_move_timer.just_finished() {
        match World::write_state(
            &game_state.er_client,
            &game_state.keypair,
            &game_state.state_name,
            &game_state.position,
        ) {
            Ok(hash) => println!("Synced position to ER: {}", hash),
            Err(e) => eprintln!("Failed to sync to ER: {}", e),
        }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
pub struct MyPosition {
    x: f32,
    y: f32,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
pub struct MyHealth {
    health: u16,
}
