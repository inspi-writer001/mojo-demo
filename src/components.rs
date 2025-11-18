use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub enum GameOverButton {
    Restart,
    Exit,
}

