pub mod gas_giant;
pub mod terran_wet;

use bevy::{prelude::*, sprite::Material2dPlugin};

use crate::menu::MenuState;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			Material2dPlugin::<gas_giant::GasGiantMaterial>::default(),
			// Material2dPlugin::<terran_wet::TerranWetMaterial>::default()
		))
		.add_systems(OnEnter(MenuState::Main), gas_giant::setup);
	}
}
