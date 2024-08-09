pub mod base;
pub mod factory;
mod stat;

use std::collections::HashMap;

use bevy::prelude::*;
use stat::PlanetStat;

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
	fn build(&self, _app: &mut App) {
		for planet in base::BasePlanet::iter() {
			info!("planet {:?}", factory::PlanetFactory::Base(planet).stat())
		}
		info!("Built PlanetPlugin")
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
