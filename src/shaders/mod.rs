pub mod clouds;
pub mod land_rivers;

use bevy::{prelude::*, sprite::Material2dPlugin};
use clouds::CloudShader;
use land_rivers::LandRiversShader;

pub struct ShadersPlugin;

macro_rules! register_shaders {
	($($shader:ident),*) => {
		impl Plugin for ShadersPlugin {
			fn build(&self, app: &mut App) {
				app.add_plugins(($(
					Material2dPlugin::<$shader>::default()
				),*))
				$(
					.register_asset_reflect::<$shader>()
				)*;
			}
		}
	}
}

register_shaders!(CloudShader, LandRiversShader);
