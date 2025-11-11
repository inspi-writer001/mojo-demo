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
struct PlayerKeypair(Keypair);

#[derive(Resource)]
struct PlayerStateName(String);

#[derive(Resource, Clone, Copy)]
struct PlayerPosition(MyPosition);

#[derive(Resource)]
struct ErClient(WorldClient);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let payer = read_keypair_file("dev_wallet-keypair.json").expect("Couldn't find wallet file");

    let rpc_client = WorldClient::new("https://api.devnet.solana.com");
    let er_rpc_client = WorldClient::new("https://devnet.magicblock.app/");

    let new_world = World::create_world(&rpc_client, &payer, "moving_game16");
    let state_name = "brother_position16";
    println!("we got world yayy, {}", new_world.unwrap());

    let player_position = MyPosition { x: 0.0, y: 0.0 };
    let new_player =
        World::create_state::<MyPosition>(&rpc_client, &payer, &state_name, &player_position);
    println!(
        "we just spawned a character position and delegated, {}",
        new_player.unwrap()
    );

    // Insert resources so they can be accessed in systems
    commands.insert_resource(PlayerKeypair(payer));
    commands.insert_resource(PlayerStateName(state_name.to_string()));
    commands.insert_resource(PlayerPosition(player_position));
    commands.insert_resource(ErClient(er_rpc_client));

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
    keypair: Res<PlayerKeypair>,
    state_name: Res<PlayerStateName>,
    mut player_position: ResMut<PlayerPosition>,
    rpc_client: Res<ErClient>,
) {
    for (mut transform, _) in &mut characters {
        // do the state update here on character movement
        if input.pressed(KeyCode::Up) {
            transform.translation.y += 100.0 * time.delta_seconds();
            player_position.0.y += 100.0 * time.delta_seconds();
            World::write_state(&rpc_client.0, &keypair.0, &state_name.0, &player_position.0)
                .expect("failed to send tx in ER");
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= 100.0 * time.delta_seconds();
            player_position.0.y -= 100.0 * time.delta_seconds();
            World::write_state(&rpc_client.0, &keypair.0, &state_name.0, &player_position.0)
                .expect("failed to send tx in ER");
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += 100.0 * time.delta_seconds();
            player_position.0.x += 100.0 * time.delta_seconds();
            World::write_state(&rpc_client.0, &keypair.0, &state_name.0, &player_position.0)
                .expect("failed to send tx in ER");
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= 100.0 * time.delta_seconds();
            player_position.0.x -= 100.0 * time.delta_seconds();
            World::write_state(&rpc_client.0, &keypair.0, &state_name.0, &player_position.0)
                .expect("failed to send tx in ER");
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
