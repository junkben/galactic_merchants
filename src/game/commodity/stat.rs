use convert_case::{Case, Casing};

#[derive(Debug, Clone, Copy)]
pub struct CommodityStat {
	pub name:        &'static str,
	pub price_range: (u32, u32)
}

impl CommodityStat {
	fn name(&self) -> String { self.name.to_case(Case::Title) }

	fn price_min(&self) -> u32 { self.price_range.0 }

	fn price_max(&self) -> u32 { self.price_range.1 }
}
