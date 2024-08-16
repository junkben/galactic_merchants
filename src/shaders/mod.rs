pub mod gas_giant;

use bevy::{prelude::*, sprite::Material2dPlugin};

use crate::menu::MenuState;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(
			Material2dPlugin::<gas_giant::GasGiantMaterial>::default()
		)
		.register_asset_reflect::<gas_giant::GasGiantMaterial>()
		.add_systems(OnEnter(MenuState::Main), gas_giant::setup);
	}
}
