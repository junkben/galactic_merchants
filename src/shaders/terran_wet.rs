use bevy::{
	prelude::*,
	render::render_resource::{AsBindGroup, ShaderRef},
	sprite::{Material2d, MaterialMesh2dBundle}
};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

const SHADER_ASSET_PATH: &str = "shaders/planets/terran_wet.wgsl";

// Setup a simple 2d scene
pub fn setup(
	commands: Commands,
	meshes: ResMut<Assets<Mesh>>,
	materials: ResMut<Assets<GasGiantMaterial>>
) {
	spawn_nosh(commands, meshes, materials);
}

#[derive(Bundle)]
pub struct GasGiantBundle {
	cloud_a: MaterialMesh2dBundle<GasGiantMaterial>,
	cloud_b: MaterialMesh2dBundle<GasGiantMaterial>
}

impl GasGiantBundle {
	pub fn spawn_default(
		commands: Commands,
		meshes: ResMut<Assets<Mesh>>,
		materials: ResMut<Assets<GasGiantMaterial>>
	) {
		Self::spawn(GasGiantConfig::default(), commands, meshes, materials)
	}

	pub fn spawn(
		config: GasGiantConfig,
		mut commands: Commands,
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<GasGiantMaterial>>
	) {
		// spawn inner cloud
		commands.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(Rectangle::default()).into(),
			transform: config.transform.with_translation(Vec3 {
				x: 0.0,
				y: 0.0,
				z: -1.0
			}),
			material: materials.add(config.inner_cloud_material()),
			..default()
		});

		// spawn outer cloud
		commands.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(Rectangle::default()).into(),
			transform: config.transform,
			material: materials.add(config.outer_cloud_material()),
			..default()
		});
	}
}

pub struct GasGiantConfig {
	transform:          Transform,
	pixels:             f32,
	light_origin:       Vec2,
	time_speed:         f32,
	stretch:            f32,
	cloud_curve:        f32,
	rotation:           f32,
	size:               f32,
	seed:               f32,
	inner_cloud_config: GasGiantCloudConfig,
	outer_cloud_config: GasGiantCloudConfig
}

impl Default for GasGiantConfig {
	fn default() -> Self {
		let colors: [LinearRgba; 8] = [
			LinearRgba::from_u8_array([101, 0, 0, 255]),
			LinearRgba::from_u8_array([101, 0, 0, 255]),
			LinearRgba::from_u8_array([79, 3, 1, 255]),
			LinearRgba::from_u8_array([79, 3, 1, 255]),
			LinearRgba::from_u8_array([246, 152, 0, 255]),
			LinearRgba::from_u8_array([201, 108, 0, 255]),
			LinearRgba::from_u8_array([171, 72, 7, 255]),
			LinearRgba::from_u8_array([85, 12, 0, 255])
		];
		Self {
			transform:          Transform::default()
				.with_scale(Vec3::splat(400.)),
			pixels:             100.0,
			light_origin:       Vec2 { x: 0.25, y: 0.25 },
			time_speed:         0.47,
			stretch:            1.0,
			cloud_curve:        1.3,
			rotation:           0.0,
			size:               9.0,
			seed:               5.939,
			inner_cloud_config: GasGiantCloudConfig {
				cloud_cover:          0.0,
				light_border_1:       0.692,
				light_border_2:       0.666,
				base_color:           colors[0],
				outline_color:        colors[1],
				shadow_base_color:    colors[2],
				shadow_outline_color: colors[3]
			},
			outer_cloud_config: GasGiantCloudConfig {
				cloud_cover:          0.538,
				light_border_1:       0.439,
				light_border_2:       0.746,
				base_color:           colors[4],
				outline_color:        colors[5],
				shadow_base_color:    colors[6],
				shadow_outline_color: colors[7]
			}
		}
	}
}

fn spawn_nosh(
	commands: Commands,
	meshes: ResMut<Assets<Mesh>>,
	materials: ResMut<Assets<GasGiantMaterial>>
) {
	let colors: [LinearRgba; 8] = [
		LinearRgba::from_u8_array([101, 0, 0, 255]),
		LinearRgba::from_u8_array([101, 0, 0, 255]),
		LinearRgba::from_u8_array([79, 3, 1, 255]),
		LinearRgba::from_u8_array([79, 3, 1, 255]),
		LinearRgba::from_u8_array([246, 152, 0, 255]),
		LinearRgba::from_u8_array([201, 108, 0, 255]),
		LinearRgba::from_u8_array([171, 72, 7, 255]),
		LinearRgba::from_u8_array([85, 12, 0, 255])
	];
	GasGiantBundle::spawn(
		GasGiantConfig {
			pixels: 128.0,
			size: 15.0,
			time_speed: 0.1,
			inner_cloud_config: GasGiantCloudConfig {
				light_border_1:       0.7,
				light_border_2:       0.85,
				cloud_cover:          0.00,
				base_color:           colors[0],
				outline_color:        colors[1],
				shadow_base_color:    colors[2],
				shadow_outline_color: colors[3]
			},
			outer_cloud_config: GasGiantCloudConfig {
				light_border_1:       0.7,
				light_border_2:       0.85,
				cloud_cover:          0.35,
				base_color:           colors[4],
				outline_color:        colors[5],
				shadow_base_color:    colors[6],
				shadow_outline_color: colors[7]
			},
			..default()
		},
		commands,
		meshes,
		materials
	)
}

impl GasGiantConfig {
	fn inner_cloud_material(&self) -> GasGiantMaterial {
		GasGiantMaterial {
			pixels:         self.pixels,
			cloud_cover:    self.inner_cloud_config.cloud_cover,
			light_origin:   self.light_origin,
			time_speed:     self.time_speed,
			stretch:        self.stretch,
			cloud_curve:    self.cloud_curve,
			light_border_1: self.inner_cloud_config.light_border_1,
			light_border_2: self.inner_cloud_config.light_border_2,
			rotation:       self.rotation,
			color_a:        self.inner_cloud_config.base_color,
			color_b:        self.inner_cloud_config.outline_color,
			color_c:        self.inner_cloud_config.shadow_base_color,
			color_d:        self.inner_cloud_config.shadow_outline_color,
			size:           self.size,
			seed:           self.seed
		}
	}

	fn outer_cloud_material(&self) -> GasGiantMaterial {
		GasGiantMaterial {
			pixels:         self.pixels,
			cloud_cover:    self.outer_cloud_config.cloud_cover,
			light_origin:   self.light_origin,
			time_speed:     self.time_speed,
			stretch:        self.stretch,
			cloud_curve:    self.cloud_curve,
			light_border_1: self.outer_cloud_config.light_border_1,
			light_border_2: self.outer_cloud_config.light_border_2,
			rotation:       self.rotation,
			color_a:        self.outer_cloud_config.base_color,
			color_b:        self.outer_cloud_config.outline_color,
			color_c:        self.outer_cloud_config.shadow_base_color,
			color_d:        self.outer_cloud_config.shadow_outline_color,
			size:           self.size,
			seed:           self.seed
		}
	}
}

pub struct GasGiantCloudConfig {
	cloud_cover:          f32,
	light_border_1:       f32,
	light_border_2:       f32,
	base_color:           LinearRgba,
	outline_color:        LinearRgba,
	shadow_base_color:    LinearRgba,
	shadow_outline_color: LinearRgba
}

// This struct defines the data that will be passed to your shader
#[derive(Reflect, Asset, AsBindGroup, Debug, Clone, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct GasGiantMaterial {
	#[uniform(0)]
	#[inspector(min = 16.0, max = 400.0)]
	pixels: f32,

	#[uniform(1)]
	#[inspector(min = 0.0, max = 1.0)]
	cloud_cover: f32,

	#[uniform(2)]
	light_origin: Vec2,

	#[uniform(3)]
	#[inspector(min = -1.0, max = 1.0)]
	time_speed: f32,

	#[uniform(4)]
	#[inspector(min = 1.0, max = 3.0)]
	stretch: f32,

	#[uniform(5)]
	#[inspector(min = 1.0, max = 2.0)]
	cloud_curve: f32,

	#[uniform(6)]
	#[inspector(min = 0.0, max = 1.0)]
	light_border_1: f32,

	#[uniform(7)]
	#[inspector(min = 0.0, max = 1.0)]
	light_border_2: f32,

	#[uniform(8)]
	#[inspector(min = 0.0, max = 6.28)]
	rotation: f32,

	#[uniform(9)]
	color_a: LinearRgba,
	#[uniform(10)]
	color_b: LinearRgba,
	#[uniform(11)]
	color_c: LinearRgba,
	#[uniform(12)]
	color_d: LinearRgba,
	#[uniform(13)]
	size:    f32,
	//#[uniform(14)]
	// octaves:        i32,
	#[uniform(14)]
	seed:    f32
}

/// The Material trait is very configurable, but comes with sensible defaults
/// for all methods. You only need to implement functions for features that need
/// non-default behavior. See the Material api docs for details!
impl Material2d for GasGiantMaterial {
	fn fragment_shader() -> ShaderRef { SHADER_ASSET_PATH.into() }
}
