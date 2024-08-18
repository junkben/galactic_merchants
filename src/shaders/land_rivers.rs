use bevy::{
	prelude::*,
	render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
	sprite::Material2d
};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

const SHADER_ASSET_PATH: &str = "shaders/planets/land_rivers.wgsl";

/// This struct defines the data that will be passed to your shader
#[derive(Asset, AsBindGroup, Debug, Clone, Copy, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct LandRiversShader {
	#[uniform(0)]
	pub config: LandRiversConfig,
	#[uniform(1)]
	pub colors: LandRiversColors
}

#[derive(ShaderType, Debug, Clone, Copy, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
#[repr(align(16))]
pub struct LandRiversConfig {
	#[inspector(min = 16.0, max = 400.0, speed = 1.0)]
	pub pixels:       f32,
	#[inspector(min = 0.0, max = 6.28, speed = 0.0628)]
	pub rotation:     f32,
	pub light_origin: Vec2,

	#[inspector(min = -1.0, max = 1.0, speed = 0.01)]
	pub time_speed:     f32,
	#[inspector(min = 0.0, max = 10.0)]
	pub dither_size:    f32,
	#[inspector(min = 0, max = 1, speed = 1.0)]
	pub should_dither:  u32,
	#[inspector(min = 0.0, max = 1.0, speed = 0.01)]
	pub light_border_1: f32,

	#[inspector(min = 0.0, max = 1.0, speed = 0.01)]
	pub light_border_2: f32,
	#[inspector(min = 0.0, max = 1.0)]
	pub river_cutoff:   f32,
	#[inspector(min = 1.0, max = 100.0, speed = 1.0)]
	pub size:           f32,
	#[inspector(min = 1, max = 10)]
	pub octaves:        u32,

	#[inspector(min = 1.0, max = 10.0, speed = 0.001)]
	pub seed: f32
}

#[derive(Debug, Clone, Copy, InspectorOptions, ShaderType, Reflect)]
#[reflect(InspectorOptions)]
#[repr(align(16))]
pub struct LandRiversColors {
	pub a: LinearRgba,
	pub b: LinearRgba,
	pub c: LinearRgba,
	pub d: LinearRgba,
	pub e: LinearRgba,
	pub f: LinearRgba
}

impl Material2d for LandRiversShader {
	fn fragment_shader() -> ShaderRef { SHADER_ASSET_PATH.into() }
}

impl Default for LandRiversShader {
	/// from `Rivers.tscn`
	fn default() -> Self {
		LandRiversShader {
			config: LandRiversConfig {
				pixels:         100.0,
				rotation:       0.2,
				light_origin:   Vec2 { x: 0.39, y: 0.39 },
				time_speed:     0.1,
				dither_size:    3.951,
				should_dither:  0,
				light_border_1: 0.287,
				light_border_2: 0.476,
				river_cutoff:   0.368,
				size:           4.6,
				octaves:        6,
				seed:           8.8
			},
			colors: LandRiversColors {
				a: LinearRgba::rgb(0.388, 0.670, 0.247),
				b: LinearRgba::rgb(0.231, 0.490, 0.309),
				c: LinearRgba::rgb(0.184, 0.341, 0.325),
				d: LinearRgba::rgb(0.156, 0.207, 0.250),
				e: LinearRgba::rgb(0.309, 0.643, 0.721),
				f: LinearRgba::rgb(0.250, 0.286, 0.450)
			}
		}
	}
}
