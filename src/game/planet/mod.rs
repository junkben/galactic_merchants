pub mod base;
pub mod bundle;
pub mod factory;
mod stat;

use std::collections::HashMap;

use bevy::prelude::*;
use bundle::{gas_giant::SpawnGasGiant, terran_wet::SpawnTerranWet};
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
	mut meshes: ResMut<Assets<Mesh>>,
	mut land_materials: ResMut<Assets<LandRiversShader>>,
	mut cloud_materials: ResMut<Assets<CloudShader>>
) {
	// Spawn terran_wet
	let (land, cloud) = SpawnTerranWet {
		transform: Transform::default()
			.with_translation(Vec3 {
				x: -225.0,
				y: 0.0,
				z: 0.0
			})
			.with_scale(Vec3::splat(400.0)),
		..default()
	}
	.to_sprite_bundles(
		meshes.as_mut(),
		land_materials.as_mut(),
		cloud_materials.as_mut()
	);

	commands.spawn(land);
	commands.spawn(cloud);

	// Spawn gas_giant_1
	let gas_giant_bundles = SpawnGasGiant {
		transform: Transform::default()
			.with_translation(Vec3 {
				x: 225.0,
				y: 0.0,
				z: 0.0
			})
			.with_scale(Vec3::splat(400.0)),
		..default()
	}
	.to_sprite_bundles(meshes.as_mut(), cloud_materials.as_mut());

	for bundle in gas_giant_bundles {
		commands.spawn(bundle);
	}
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
