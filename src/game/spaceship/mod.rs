use base::BaseSpaceship;
use bay::SpaceshipBay;
use bevy::prelude::*;
use factory::SpaceshipFactory;
use stat::SpaceshipStat;

pub mod base;
mod bay;
pub mod factory;
mod stat;

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
	fn build(&self, _app: &mut App) {
		for ship in BaseSpaceship::iter() {
			info!("spaceship {:?}", SpaceshipFactory::Base(ship).generate())
		}
		info!("Built SpaceshipPlugin")
	}
}

#[derive(Debug, Clone)]
pub struct StandardSpaceship {
	name:        String,
	description: String,
	cargo:       bay::CargoBay,
	crew:        bay::CrewQuarters,
	engine:      bay::Engines,
	fuel_tank:   bay::FuelTank,
	guests:      bay::GuestQuarters
}

impl StandardSpaceship {
	fn new(stat: &SpaceshipStat) -> Self {
		return StandardSpaceship {
			name:        stat.name.to_string(),
			description: stat.description.to_string(),
			cargo:       bay::CargoBay::new(stat.cargo_size),
			crew:        bay::CrewQuarters::new(stat.crew_size),
			engine:      bay::Engines::new(stat.engine_power),
			fuel_tank:   bay::FuelTank::new(stat.fuel_tank),
			guests:      bay::GuestQuarters::new(stat.passenger_max)
		};
	}
}

pub trait Spaceship {
	fn name(&self) -> &'static str;
	fn description(&self) -> &'static str;
	fn cargo_size(&self) -> u32;
	fn crew_size(&self) -> u32;
	fn engine_power(&self) -> u32;
	fn fuel_tank(&self) -> u32;
	fn passenger_max(&self) -> u32;
}
