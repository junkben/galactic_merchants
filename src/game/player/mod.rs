use bevy::prelude::*;

use super::spaceship::StandardSpaceship;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, _app: &mut App) {}
}

/// A component that represents a Player
#[derive(Component, Debug, Clone)]
pub struct Player {
	name:      &'static str,
	money:     u32,
	spaceship: StandardSpaceship
}
