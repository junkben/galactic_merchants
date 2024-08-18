pub mod base;
pub mod bundle;
pub mod factory;
mod stat;

use std::collections::HashMap;

use bevy::prelude::*;
use bundle::terran_wet::SpawnTerranWet;
use stat::PlanetStat;

use crate::{
	menu::MenuState,
	shaders::{clouds::CloudShader, land_rivers::LandRiversShader}
};

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(base::BasePlanetPlugin)
			.add_systems(OnEnter(MenuState::Main), spawn_planet_of_the_day);

		for planet in base::BasePlanet::iter() {
			info!("planet {:?}", factory::PlanetFactory::Base(planet).stat())
		}
		info!("Built PlanetPlugin")
	}
}

fn spawn_planet_of_the_day(
	mut commands: Commands,
	meshes: ResMut<Assets<Mesh>>,
	land_materials: ResMut<Assets<LandRiversShader>>,
	cloud_materials: ResMut<Assets<CloudShader>>
) {
	let (land, cloud) = SpawnTerranWet::default().to_sprite_bundles(
		meshes,
		land_materials,
		cloud_materials
	);

	commands.spawn(land);
	commands.spawn(cloud);
}

#[derive(Debug, Clone)]
pub struct StandardPlanet {
	name:        String,
	description: String,
	market:      Market
}

impl StandardPlanet {
	fn new(stat: &PlanetStat) -> Self {
		return StandardPlanet {
			name:        stat.name.to_string(),
			description: stat.description.to_string(),
			market:      Market::new()
		};
	}
}

#[derive(Debug, Clone)]
pub struct Market {
	prices:   HashMap<&'static str, u32>,
	supplies: HashMap<&'static str, u32>
}

impl Market {
	fn new() -> Self {
		Market {
			prices:   HashMap::new(),
			supplies: HashMap::new()
		}
	}

	fn price(&self, commodity: &'static str) -> &u32 {
		self.prices.get(commodity).unwrap()
	}

	fn supply(&self, commodity: &'static str) -> &u32 {
		self.supplies.get(commodity).unwrap()
	}
}
