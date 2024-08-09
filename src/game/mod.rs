use bevy::prelude::*;

pub mod commodity;
pub mod galaxy;
pub mod planet;
pub mod player;
pub mod spaceship;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			// Game-related plugins
			.add_plugins((
				commodity::CommodityPlugin,
				spaceship::SpaceshipPlugin,
				planet::PlanetPlugin,
				galaxy::GalaxyPlugin
			));
	}
}
