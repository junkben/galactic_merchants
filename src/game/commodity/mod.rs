use bevy::prelude::*;

mod base;
mod factory;
mod stat;

pub struct CommodityPlugin;
impl Plugin for CommodityPlugin {
	fn build(&self, _app: &mut App) {
		for com in base::BaseCommodity::iter() {
			info!(
				"commodity {:?}",
				factory::CommodityFactory::Base(com).stat()
			)
		}
		info!("Built SpaceshipPlugin")
	}
}
