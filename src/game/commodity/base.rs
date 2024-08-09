macro_rules! base_commodities {
	($($name:ident, $min:expr, $max:expr);*) => {
        use crate::game::commodity::stat::CommodityStat;

        #[derive(Debug, Clone, Copy)]
        pub enum BaseCommodity {
            $($name),*
        }

        impl BaseCommodity {
            pub fn iter() -> impl Iterator<Item = BaseCommodity> {
                use BaseCommodity::*;
                [$($name),*].iter().copied()
            }

            pub const fn stat(&self) -> &CommodityStat {
                use BaseCommodity::*;
                match self {
                    $(&$name => &CommodityStat {
                        name:	  stringify!($name),
                        price_range: ($min, $max)
                    }),*
                }
            }
        }
	};
}

base_commodities!(
	Cantaloupe, 15,  40;
	JellyBeans, 30,  80;
	MoonFerns,  45,  120;
	FrogLegs,   60,  160;
	WhipCream,  75,  200;
	BabelSeeds, 90,  240;
	Diapers,    105, 280;
	Umbrellas,  120, 320;
	Toasters,   135, 360;
	Polyester,  150, 400;
	HairTonic,  165, 440;
	LavaLamps,  180, 480;
	Oxygen,     195, 520;
	OggleSand,  210, 560;
	Kryptoons,  225, 600;
	XFuels,     240, 640;
	Gems,       255, 680;
	Exotic,     270, 720
);
