use bevy::prelude::*;
use mojo_rust_sdk::{client::WorldClient, world::World};
use solana_keypair::Keypair;

use crate::types::{MyHealth, MyPosition};

#[derive(Resource)]
pub struct GameState {
    pub world: World,
    pub keypair: Keypair,
    pub position_state_name: String,
    pub health_state_name: String,
    pub position: MyPosition,
    pub health: MyHealth,
    pub last_sync_timer: Timer,
}
