use super::{base::BaseCommodity, stat::CommodityStat};

#[derive(Debug, Clone, Copy)]
pub enum CommodityFactory {
	Base(BaseCommodity),
	Custom(CommodityStat)
}

impl CommodityFactory {
	pub const fn stat(&self) -> &CommodityStat {
		use CommodityFactory::*;
		match self {
			Base(com) => com.stat(),
			Custom(stat) => stat
		}
	}
}
