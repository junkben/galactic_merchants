use rand::{seq::IteratorRandom, thread_rng};

use super::{stat::GalaxyStat, StandardGalaxy};
use crate::game::planet::{base::BasePlanet, factory::PlanetFactory};

#[derive(Debug, Clone, Copy)]
pub enum GalaxyFactory {
	Base,
	Custom(GalaxyStat)
}

impl GalaxyFactory {
	pub const fn stat(&self) -> &GalaxyStat {
		use GalaxyFactory::*;
		match self {
			Base => &GalaxyStat {
				name:        "Kukubia",
				description: "Description of Kukubia.",
				num_planets: 7
			},
			Custom(stat) => stat
		}
	}

	pub fn generate(&self) -> StandardGalaxy {
		let mut rng = thread_rng();
		let stat = self.stat();

		let planets = BasePlanet::iter()
			.choose_multiple(&mut rng, stat.num_planets)
			.iter()
			.map(|&base_planet| PlanetFactory::Base(base_planet).generate())
			.collect::<Vec<_>>();

		StandardGalaxy {
			name: stat.name.to_string(),
			description: stat.description.to_string(),
			planets
		}
	}
}
