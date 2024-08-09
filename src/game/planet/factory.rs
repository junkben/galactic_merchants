use super::{base::BasePlanet, stat::PlanetStat, StandardPlanet};

#[derive(Debug, Clone, Copy)]
pub enum PlanetFactory {
	Base(BasePlanet),
	Custom(PlanetStat)
}

impl PlanetFactory {
	pub const fn stat(&self) -> &PlanetStat {
		use PlanetFactory::*;
		match self {
			Base(base_planet) => base_planet.stat(),
			Custom(stat) => stat
		}
	}

	pub fn generate(&self) -> StandardPlanet {
		StandardPlanet::new(self.stat())
	}
}
