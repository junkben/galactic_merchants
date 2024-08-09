pub mod factory;
mod stat;

use bevy::prelude::*;

use super::planet::StandardPlanet;

pub struct GalaxyPlugin;
impl Plugin for GalaxyPlugin {
	fn build(&self, _app: &mut App) {
		info!("galaxy {:?}", factory::GalaxyFactory::Base.generate());
		info!("Built GalaxyPlugin")
	}
}

#[derive(Debug, Clone)]
pub struct StandardGalaxy {
	name:        String,
	description: String,
	planets:     Vec<StandardPlanet>
}
