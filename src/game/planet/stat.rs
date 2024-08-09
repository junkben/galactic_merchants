use convert_case::{Case, Casing};

#[derive(Debug, Clone, Copy)]
pub struct PlanetStat {
	pub name:        &'static str,
	pub description: &'static str
}

impl PlanetStat {
	fn name(&self) -> String { self.name.to_case(Case::Title) }
}
