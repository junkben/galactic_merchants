use convert_case::{Case, Casing};

#[derive(Debug, Clone, Copy)]
pub struct GalaxyStat {
	pub name:        &'static str,
	pub description: &'static str,
	pub num_planets: usize
}

impl GalaxyStat {
	fn name(&self) -> String { self.name.to_case(Case::Title) }
}
