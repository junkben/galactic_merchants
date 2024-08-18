use bevy::{
	prelude::*,
	render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
	sprite::Material2d
};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

const SHADER_ASSET_PATH: &str = "shaders/planets/clouds.wgsl";

/// This struct defines the data that will be passed to your shader
#[derive(Asset, AsBindGroup, Debug, Clone, Copy, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct CloudShader {
	#[uniform(0)]
	pub config: CloudConfig,
	#[uniform(1)]
	pub colors: CloudColors
}

#[derive(ShaderType, Debug, Clone, Copy, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
#[repr(align(16))]
pub struct CloudConfig {
	#[inspector(min = 16.0, max = 400.0, speed = 1.0)]
	pub pixels:         f32,
	#[inspector(min = 0.0, max = 1.0, speed = 0.01)]
	pub cloud_cover:    f32,
	pub light_origin:   Vec2,
	#[inspector(min = -1.0, max = 1.0, speed = 0.01)]
	pub time_speed:     f32,
	#[inspector(min = 1.0, max = 3.0)]
	pub stretch:        f32,
	#[inspector(min = 1.0, max = 2.0)]
	pub cloud_curve:    f32,
	#[inspector(min = 0.0, max = 1.0, speed = 0.01)]
	pub light_border_1: f32,
	#[inspector(min = 0.0, max = 1.0, speed = 0.01)]
	pub light_border_2: f32,
	#[inspector(min = 0.0, max = 6.28, speed = 0.0628)]
	pub rotation:       f32,
	#[inspector(min = 1.0, max = 100.0, speed = 1.0)]
	pub size:           f32,
	#[inspector(min = 1, max = 10)]
	pub octaves:        u32,
	#[inspector(min = 1.0, max = 10.0, speed = 0.001)]
	pub seed:           f32
}

#[derive(Debug, Clone, Copy, InspectorOptions, ShaderType, Reflect)]
#[reflect(InspectorOptions)]
#[repr(align(16))]
pub struct CloudColors {
	pub base:           LinearRgba,
	pub outline:        LinearRgba,
	pub shadow_base:    LinearRgba,
	pub shadow_outline: LinearRgba
}

impl Material2d for CloudShader {
	fn fragment_shader() -> ShaderRef { SHADER_ASSET_PATH.into() }
}

impl Default for CloudShader {
	/// from `GasPlanet.tscn`
	fn default() -> Self {
		CloudShader {
			config: CloudConfig {
				pixels:         100.0,
				cloud_cover:    0.538,
				light_origin:   Vec2 { x: 0.25, y: 0.25 },
				time_speed:     0.47,
				stretch:        1.0,
				cloud_curve:    1.3,
				light_border_1: 0.692,
				light_border_2: 0.666,
				rotation:       0.0,
				size:           9.0,
				octaves:        5,
				seed:           5.939
			},
			colors: CloudColors {
				base:           LinearRgba::from_u8_array([246, 152, 0, 255]),
				outline:        LinearRgba::from_u8_array([201, 108, 0, 255]),
				shadow_base:    LinearRgba::from_u8_array([171, 72, 7, 255]),
				shadow_outline: LinearRgba::from_u8_array([85, 12, 0, 255])
			}
		}
	}
}
