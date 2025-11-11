use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bytemuck::{Pod, Zeroable};
use mojo_rust_sdk::{client::WorldClient, world::World};
use solana_keypair::read_keypair_file;

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let payer = read_keypair_file("dev_wallet-keypair.json").expect("Couldn't find wallet file");

    let rpc_client = WorldClient::new("https://api.devnet.solana.com");

    let new_world = World::create_world(&rpc_client, &payer, "moving_game1");
    println!("we got world yayy, {}", new_world.unwrap());

    let player_position = MyPosition { x: 0.0, y: 0.0 };
    let new_player =
        World::create::<MyPosition>(&rpc_client, &payer, "brother_position", &player_position);
    println!(
        "we just spawned a character position, {}",
        new_player.unwrap()
    );

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
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::Up) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= 100.0 * time.delta_seconds();
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
