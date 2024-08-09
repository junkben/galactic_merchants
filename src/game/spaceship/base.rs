use BaseSpaceship::*;

use super::stat::SpaceshipStat;

#[derive(Debug, Clone, Copy)]
pub enum BaseSpaceship {
	StingerXII,
	FlyCatcher,
	LeRock,
	Whaler2000,
	Retina,
	Cerebralis,
	Globulizer,
	Locomotis,
	Mantagon,
	Keggar,
	WormShuttle,
	Squidocity
}

impl BaseSpaceship {
	pub fn iter() -> impl Iterator<Item = BaseSpaceship> {
		[
			StingerXII,
			FlyCatcher,
			LeRock,
			Whaler2000,
			Retina,
			Cerebralis,
			Globulizer,
			Locomotis,
			Mantagon,
			Keggar,
			WormShuttle,
			Squidocity
		]
		.iter()
		.copied()
	}

	pub const fn base_stat(&self) -> &SpaceshipStat {
		match self {
			StingerXII => &STINGER_XII,
			FlyCatcher => &FLY_CATCHER,
			LeRock => &LE_ROCK,
			Whaler2000 => &WHALER_2000,
			Retina => &RETINA,
			Cerebralis => &CEREBRALIS,
			Globulizer => &GLOBULIZER,
			Locomotis => &LOCOMOTIS,
			Mantagon => &MANTAGON,
			Keggar => &KEGGAR,
			WormShuttle => &WORM_SHUTTLE,
			Squidocity => &SQUIDOCITY
		}
	}
}

pub const STINGER_XII: SpaceshipStat = SpaceshipStat {
	name:          "The Stinger XII",
	description:   "The Stinger XII costs 110,000 kubars and is a speed \
	                demon. It comes with a high-powered 7-kuarp engine, \
	                requires 4 crew members to operate, and can carry up to \
	                100 tons of cargo and 8 passengers. The one disadvantage \
	                is that the fuel tank is only 20 tons.",
	cargo_size:    100,
	crew_size:     4,
	engine_power:  7,
	fuel_tank:     20,
	passenger_max: 8
};

pub const FLY_CATCHER: SpaceshipStat = SpaceshipStat {
	name:          "The Fly Catcher",
	description:   "The Fly Catcher costs 110,000 kubars and is designed \
	                especially for hauling cargo. It can carry up to 120 tons \
	                of cargo and comes with a 40-ton fuel tank, a 5-kuarp \
	                engine and room for 8 passengers. The one disadvantage is \
	                that it requires 5 crew members to operate.",
	cargo_size:    120,
	crew_size:     5,
	engine_power:  5,
	fuel_tank:     40,
	passenger_max: 8
};

pub const LE_ROCK: SpaceshipStat = SpaceshipStat {
	name:          "Le Rock",
	description:   "Le Rock costs 110,000 kubars and is sure to please even \
	                the most frugal merchants. It requires only 3 crew \
	                members and has a 65-ton fuel tank, a 5-kuarp engine and \
	                room for 8 passengers. The one disadvantage is that it \
	                can hold only 80 tons of cargo.",
	cargo_size:    80,
	crew_size:     3,
	engine_power:  5,
	fuel_tank:     65,
	passenger_max: 8
};

pub const WHALER_2000: SpaceshipStat = SpaceshipStat {
	name:          "Whaler 2000",
	description:   "Whaler 2000 is a monster of a ship. It costs 110,000 \
	                kubars and can carry up to 130 tons of cargo, 50 tons of \
	                fuel and 11 passengers. The disadvantages are that it \
	                only has a 2-kuarp engine and requires 6 crew members to \
	                operate.",
	cargo_size:    130,
	crew_size:     6,
	engine_power:  2,
	fuel_tank:     50,
	passenger_max: 11
};

pub const RETINA: SpaceshipStat = SpaceshipStat {
	name:          "Retina",
	description:   "Retina costs 110,000 kubars and is a sleek, slim ship. It \
	                requires only 3 crew members to operate and comes with a \
	                5-kuarp engine, 40-ton fuel tank and 100-ton cargo \
	                capacity. The one disadvantage is that it only has room \
	                for 6 passengers.",
	cargo_size:    100,
	crew_size:     3,
	engine_power:  5,
	fuel_tank:     40,
	passenger_max: 6
};

pub const CEREBRALIS: SpaceshipStat = SpaceshipStat {
	name:          "Cerabralis",
	description:   "Cerebralis costs 110,000 kubars and is the industry \
	                standard. It comes equipped with 100 tons of cargo \
	                capacity, a 5-kuarp engine, a 40-ton fuel tank, 4 crew \
	                members and room for 8 passengers.",
	cargo_size:    100,
	crew_size:     4,
	engine_power:  5,
	fuel_tank:     40,
	passenger_max: 8
};

pub const GLOBULIZER: SpaceshipStat = SpaceshipStat {
	name:          "The Globulizer",
	description:   "The Globulizer costs 110,000 kubars and is incredibly \
	                lightweight and quick. It has a turbocharged 7-kuarp \
	                engine, requires 4 crew members to operate, and comes \
	                with a 30-ton fuel tank and seating for 7 passengers. The \
	                disadvantage is that it has only an 80-ton cargo bay.",
	cargo_size:    80,
	crew_size:     4,
	engine_power:  7,
	fuel_tank:     30,
	passenger_max: 7
};

pub const LOCOMOTIS: SpaceshipStat = SpaceshipStat {
	name:          "Locomotis",
	description:   "Locomotis costs 110,000 kubars and is a sturdy workhorse. \
	                It requires 4 crew members to operate and comes with a \
	                110-ton cargo bay, a powerful 6-kuarp engine, a 40-ton \
	                fuel tank and room for 5 passengers.",
	cargo_size:    110,
	crew_size:     4,
	engine_power:  6,
	fuel_tank:     40,
	passenger_max: 5
};

pub const MANTAGON: SpaceshipStat = SpaceshipStat {
	name:          "Mantagon",
	description:   "Mantagon costs 110,000 kubars and is a highly efficient \
	                and well-designed merchant ship. It can carry up to 10 \
	                passengers, has a 4-kuarp engine, a 40-ton fuel tank, and \
	                a 90-ton cargo bay. The nice thing is that it only \
	                requires 3 crew members to operate.",
	cargo_size:    90,
	crew_size:     3,
	engine_power:  4,
	fuel_tank:     40,
	passenger_max: 10
};

pub const KEGGAR: SpaceshipStat = SpaceshipStat {
	name:          "Keggar",
	description:   "Kegger costs 110,000 kubars and is a massive transport \
	                vessel. It can carry up to 150 tons of cargo and 35 tons \
	                of fuel, but only has room for 1 passenger. It requires 2 \
	                crew members and comes with a 3-kuarp engine.",
	cargo_size:    150,
	crew_size:     2,
	engine_power:  3,
	fuel_tank:     35,
	passenger_max: 1
};

pub const WORM_SHUTTLE: SpaceshipStat = SpaceshipStat {
	name:          "Worm Shuttle",
	description:   "Worm Shuttle costs 110,000 kubars and is a deluxe \
	                passenger liner. It can accommodate up to 16 passengers \
	                and comes with a speedy 6-kuarp engine and a 30-ton fuel \
	                tank. The drawbacks are that it requires 12 crew members \
	                to operate and has a small 75-ton cargo bay.",
	cargo_size:    75,
	crew_size:     12,
	engine_power:  6,
	fuel_tank:     30,
	passenger_max: 16
};

pub const SQUIDOCITY: SpaceshipStat = SpaceshipStat {
	name:          "Squidocity",
	description:   "Squidocity costs 110,000 kubars and is a smartly designed \
	                freighter. It can carry up to 110 tons of cargo and comes \
	                with a 40-ton fuel tank, room for 8 passengers, and a \
	                fast 6-kuarp engine. The one disadvantage is that it \
	                requires 6 crew members to operate.",
	cargo_size:    110,
	crew_size:     6,
	engine_power:  6,
	fuel_tank:     40,
	passenger_max: 8
};


