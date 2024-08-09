use bevy::prelude::*;

use super::{base::BaseSpaceship, stat::*, StandardSpaceship};

#[derive(Component, Debug, Clone, Copy)]
pub enum SpaceshipFactory {
	Generic,
	Base(BaseSpaceship),
	Custom(SpaceshipStat)
}

impl SpaceshipFactory {
	pub const fn stat(&self) -> &SpaceshipStat {
		use SpaceshipFactory::*;
		match self {
			Generic => &GENERIC,
			Base(ship) => ship.base_stat(),
			Custom(stat) => stat
		}
	}

	pub fn generate(&self) -> StandardSpaceship {
		StandardSpaceship::new(self.stat())
	}
}

pub const GENERIC: SpaceshipStat = SpaceshipStat {
	name:          "Custom Spaceship",
	description:   "A custom Spaceship.",
	cargo_size:    100,
	crew_size:     4,
	engine_power:  5,
	fuel_tank:     40,
	passenger_max: 8
};
