use bevy::{app::AppExit, core_pipeline::clear_color::ClearColorConfig, prelude::*};
use mojo_rust_sdk::client::RpcType;
use mojo_rust_sdk::world::World;
use solana_keypair::read_keypair_file;

use crate::components::{GameOverButton, GameOverUI, Health, HealthBarFill, Player};
use crate::resources::GameState;
use crate::states::AppState;
use crate::types::{MyHealth, MyPosition};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let payer = read_keypair_file("dev_wallet-keypair.json").expect("Couldn't find wallet file");

    let new_world = World::create_world(RpcType::Devnet, &payer, "moving_game703");
    let position_state_name = "brother_position703";
    let health_state_name = "brother_health703";
    println!("we got world yayy, {}", new_world.unwrap());

    let player_position = MyPosition { x: 0.0, y: 0.0 };
    let new_player = World::create_state::<MyPosition>(
        RpcType::Devnet,
        &payer,
        &position_state_name,
        &player_position,
    );
    println!(
        "we just spawned a character position and delegated, {}",
        new_player.unwrap()
    );

    let initial_health = MyHealth { health: 1000 };
    let new_health_state = World::create_state::<MyHealth>(
        RpcType::Devnet,
        &payer,
        &health_state_name,
        &initial_health,
    );
    println!("Created health state: {}", new_health_state.unwrap());

    commands.insert_resource(GameState {
        keypair: payer,
        position_state_name: position_state_name.to_string(),
        health_state_name: health_state_name.to_string(),
        position: player_position,
        health: initial_health,

        last_sync_timer: Timer::from_seconds(5.0, TimerMode::Once),
    });

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GREEN),
        },
        ..Default::default()
    });

    let texture = asset_server.load("character.png");
    commands.spawn((
        Player,
        Health {
            current: 100.0,
            max: 100.0,
        },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 100.0, y: 100.0 }),
                ..default()
            },
            texture,
            ..default()
        },
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(200.0),
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                HealthBarFill,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb(1.0, 0.2, 0.2)),
                    ..default()
                },
            ));
        });
}

pub fn character_movement(
    mut characters: Query<(&mut Transform, &mut Health), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
) {
    const SPEED: f32 = 100.0;
    const HEALTH_DECAY_PER_SECOND: f32 = 5.0;

    let mut has_moved = false;

    for (mut transform, mut health) in &mut characters {
        if health.current <= 0.0 {
            continue;
        }

        if input.pressed(KeyCode::Up) {
            transform.translation.y += SPEED * time.delta_seconds();
            has_moved = true;
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= SPEED * time.delta_seconds();
            has_moved = true;
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += SPEED * time.delta_seconds();
            has_moved = true;
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= SPEED * time.delta_seconds();
            has_moved = true;
        }

        if has_moved {
            health.current -= HEALTH_DECAY_PER_SECOND * time.delta_seconds();
            health.current = health.current.max(0.0);

            game_state.position.x = transform.translation.x;
            game_state.position.y = transform.translation.y;

            let health_u16 = (health.current / health.max * 1000.0) as u16;
            game_state.health.health = health_u16;
        }
    }

    if has_moved {
        game_state.last_sync_timer.reset();
    }

    game_state.last_sync_timer.tick(time.delta());

    if game_state.last_sync_timer.just_finished() {
        match World::write_state(
            RpcType::Devnet,
            &game_state.keypair,
            &game_state.position_state_name,
            &game_state.position,
        ) {
            Ok(hash) => println!("Synced position to ER: {}", hash),
            Err(e) => eprintln!("Failed to sync position: {}", e),
        }

        match World::write_state(
            RpcType::Devnet,
            &game_state.keypair,
            &game_state.health_state_name,
            &game_state.health,
        ) {
            Ok(hash) => println!(
                "Synced health to ER: {} (health: {})",
                hash, game_state.health.health
            ),
            Err(e) => eprintln!("Failed to sync health: {}", e),
        }
    }
}

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<&mut Style, With<HealthBarFill>>,
) {
    if let Ok(health) = player_query.get_single() {
        if let Ok(mut style) = health_bar_query.get_single_mut() {
            let health_percentage = (health.current / health.max * 100.0).max(0.0);
            style.width = Val::Percent(health_percentage);
        }
    }
}

pub fn check_game_over(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(health) = player_query.get_single() {
        if health.current <= 0.0 {
            next_state.set(AppState::GameOver);
        }
    }
}

pub fn show_game_over(mut commands: Commands) {
    commands
        .spawn((
            GameOverUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.8)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Game Over",
                        TextStyle {
                            font_size: 60.0,
                            color: Color::rgb(1.0, 0.2, 0.2),
                            ..default()
                        },
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    GameOverButton::Restart,
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(200.0),
                                            height: Val::Px(50.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::rgb(
                                            0.3, 0.3, 0.3,
                                        )),
                                        ..default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Restart",
                                        TextStyle {
                                            font_size: 24.0,
                                            color: Color::WHITE,
                                            ..default()
                                        },
                                    ));
                                });

                            parent
                                .spawn((
                                    GameOverButton::Exit,
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(200.0),
                                            height: Val::Px(50.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::rgb(
                                            0.3, 0.3, 0.3,
                                        )),
                                        ..default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Exit",
                                        TextStyle {
                                            font_size: 24.0,
                                            color: Color::WHITE,
                                            ..default()
                                        },
                                    ));
                                });
                        });
                });
        });
}

pub fn handle_game_over_buttons(
    mut interaction_query: Query<
        (&Interaction, &GameOverButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut commands: Commands,
    ui_query: Query<Entity, With<GameOverUI>>,
    player_query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match button {
                GameOverButton::Restart => {
                    for entity in ui_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }

                    for entity in player_query.iter() {
                        commands.entity(entity).despawn();
                    }

                    let texture = asset_server.load("character.png");
                    commands.spawn((
                        Player,
                        Health {
                            current: 100.0,
                            max: 100.0,
                        },
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2 { x: 100.0, y: 100.0 }),
                                ..default()
                            },
                            texture,
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..default()
                        },
                    ));

                    next_state.set(AppState::Playing);
                }
                GameOverButton::Exit => {
                    exit.send(AppExit);
                }
            },
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::rgb(0.5, 0.5, 0.5));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::rgb(0.3, 0.3, 0.3));
            }
        }
    }
}
