use bevy::{
	prelude::*,
	reflect::TypePath,
	render::render_resource::{AsBindGroup, ShaderRef},
	sprite::{Material2d, MaterialMesh2dBundle}
};
use rand::{thread_rng, Rng};

const SHADER_ASSET_PATH: &str = "shaders/gas_giant.wgsl";

// Setup a simple 2d scene
pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<GasGiantMaterial>>
) {
	let mut rng = thread_rng();

	// cloud_a
	// put this behind the next one
	let mut ca = cloud_a();
	ca.pixels = 200.0;
	ca.size = 18.0;

	commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(Rectangle::default()).into(),
		transform: Transform::default()
			.with_scale(Vec3::splat(400.))
			.with_translation(Vec3 {
				x: 0.0,
				y: 0.0,
				z: -1.0
			}),
		material: materials.add(ca),
		..default()
	});

	// cloud_b
	let mut cb = cloud_b();
	cb.seed = rng.gen_range(1.0..10.0);
	cb.pixels = 200.0;
	cb.size = 18.0;

	commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(Rectangle::default()).into(),
		transform: Transform::default().with_scale(Vec3::splat(400.)),
		material: materials.add(cb),
		..default()
	});
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GasGiantMaterial {
	#[uniform(0)]
	pixels:         f32,
	#[uniform(1)]
	cloud_cover:    f32,
	#[uniform(2)]
	light_origin:   Vec2,
	#[uniform(3)]
	time_speed:     f32,
	#[uniform(4)]
	stretch:        f32,
	#[uniform(5)]
	cloud_curve:    f32,
	#[uniform(6)]
	light_border_1: f32,
	#[uniform(7)]
	light_border_2: f32,
	#[uniform(8)]
	rotation:       f32,
	#[uniform(9)]
	color_a:        LinearRgba,
	#[uniform(10)]
	color_b:        LinearRgba,
	#[uniform(11)]
	color_c:        LinearRgba,
	#[uniform(12)]
	color_d:        LinearRgba,
	#[uniform(13)]
	size:           f32,
	//#[uniform(14)]
	// octaves:        i32,
	#[uniform(14)]
	seed:           f32
}

fn cloud_a() -> GasGiantMaterial {
	GasGiantMaterial {
		pixels:         100.,
		cloud_cover:    0.0,
		light_origin:   Vec2 { x: 0.25, y: 0.25 },
		time_speed:     0.7,
		stretch:        1.0,
		cloud_curve:    1.3,
		light_border_1: 0.692,
		light_border_2: 0.666,
		rotation:       0.0,
		color_a:        LinearRgba::from_u8_array([101, 0, 0, 255]),
		color_b:        LinearRgba::from_u8_array([101, 0, 0, 255]),
		color_c:        LinearRgba::from_u8_array([79, 3, 1, 255]),
		color_d:        LinearRgba::from_u8_array([79, 3, 1, 255]),
		size:           9.0,
		// octaves:        5,
		seed:           5.939
	}
}

fn cloud_b() -> GasGiantMaterial {
	GasGiantMaterial {
		pixels:         100.,
		cloud_cover:    0.538,
		light_origin:   Vec2 { x: 0.25, y: 0.25 },
		time_speed:     0.47,
		stretch:        1.0,
		cloud_curve:    1.3,
		light_border_1: 0.439,
		light_border_2: 0.746,
		rotation:       0.0,
		color_a:        LinearRgba::from_u8_array([246, 152, 0, 255]),
		color_b:        LinearRgba::from_u8_array([201, 108, 0, 255]),
		color_c:        LinearRgba::from_u8_array([171, 72, 7, 255]),
		color_d:        LinearRgba::from_u8_array([85, 12, 0, 255]),
		size:           9.0,
		// octaves:        5,
		seed:           5.939
	}
}

/// The Material trait is very configurable, but comes with sensible defaults
/// for all methods. You only need to implement functions for features that need
/// non-default behavior. See the Material api docs for details!
impl Material2d for GasGiantMaterial {
	fn fragment_shader() -> ShaderRef { SHADER_ASSET_PATH.into() }
}
