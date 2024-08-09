macro_rules! base_planet {
	($($name:ident, $description:expr);*) => {
        use crate::game::planet::stat::PlanetStat;

        #[derive(Debug, Clone, Copy)]
        pub enum BasePlanet {
            $($name),*
        }

        impl BasePlanet {
            pub fn iter() -> impl Iterator<Item = BasePlanet> {
                use BasePlanet::*;
                [$($name),*].iter().copied()
            }

            pub const fn stat(&self) -> &PlanetStat {
                use BasePlanet::*;
                match self {
                    $(&$name => &PlanetStat {
                        name:	     stringify!($name),
                        description: $description
                    }),*
                }
            }
        }
	};
}

base_planet!(
	Bass, "BASS is a playground for stock market analysts.";
	Frac, "FRAC is the headquarters of the Voyager's Insurance Company.";
	Hork, "HORK is the media capital of Kukubia.";
	Loro, "LORO is renowned throughout Kukubia as a great vacation spot.";
	Mira, "MIRA is the heart of Kukubia's religion and sacred resting spot of the Quaso Mutta.";
	Nosh, "NOSH is known as the fuel depot for the entire system.";
	Ooom, "OOOM is the mystical fortune teller's planet.";
	Pyke, "PYKE is a major industrial center and the only place to purchase new L-Tech engines.";
	Queg, "QUEG is the infamous smuggler's haven.";
	Stye, "STYE is the financial hub of the system and home of the Traders' Union.";
	Tilo, "TILO is the gambler's planet packed with wall to wall casinos.";
	Vexx, "VEXX is the capital of Kukubia and the seat of the Imperial Magistrate.";
	Xeen, "XEEN is one giant junkyard filled with spare ship parts and brilliant mechanics.";
	Zile, "ZILE is a wealthy merchant's planet and home to Mr. Zinn."
);
