use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::shaders::{
	clouds::{CloudColors, CloudConfig, CloudShader},
	land_rivers::{LandRiversColors, LandRiversConfig, LandRiversShader}
};

pub struct SpawnTerranWet {
	pub transform:    Transform,
	pub land_shader:  LandRiversShader,
	pub cloud_shader: CloudShader
}

impl Default for SpawnTerranWet {
	/// from Rivers.tscn
	fn default() -> Self {
		SpawnTerranWet {
			transform:    Transform::default().with_scale(Vec3::splat(400.0)),
			land_shader:  LandRiversShader {
				config: LandRiversConfig {
					pixels:         100.0,
					rotation:       0.2,
					light_origin:   Vec2 { x: 0.39, y: 0.39 },
					time_speed:     0.1,
					dither_size:    3.951,
					should_dither:  1u32,
					light_border_1: 0.287,
					light_border_2: 0.476,
					river_cutoff:   0.368,
					size:           4.6,
					octaves:        6,
					seed:           8.98
				},
				colors: LandRiversColors {
					a: LinearRgba::rgb(0.388, 0.670, 0.247),
					b: LinearRgba::rgb(0.231, 0.490, 0.309),
					c: LinearRgba::rgb(0.184, 0.341, 0.325),
					d: LinearRgba::rgb(0.156, 0.207, 0.250),
					e: LinearRgba::rgb(0.309, 0.643, 0.721),
					f: LinearRgba::rgb(0.250, 0.286, 0.450)
				}
			},
			cloud_shader: CloudShader {
				config: CloudConfig {
					pixels:         100.0,
					cloud_cover:    0.47,
					light_origin:   Vec2 { x: 0.39, y: 0.39 },
					time_speed:     0.1,
					stretch:        2.0,
					cloud_curve:    1.3,
					light_border_1: 0.520,
					light_border_2: 0.620,
					rotation:       0.0,
					size:           7.315,
					octaves:        2,
					seed:           5.939
				},
				colors: CloudColors {
					base:           LinearRgba::rgb(0.960, 1.000, 0.909),
					outline:        LinearRgba::rgb(0.874, 0.878, 0.909),
					shadow_base:    LinearRgba::rgb(0.407, 0.435, 0.600),
					shadow_outline: LinearRgba::rgb(0.250, 0.286, 0.450)
				}
			}
		}
	}
}

impl SpawnTerranWet {
	fn land_rivers_transform(&self) -> Transform {
		self.transform.with_translation(Vec3 {
			x: self.transform.translation.x,
			y: self.transform.translation.y,
			z: self.transform.translation.z - 1.0
		})
	}

	fn cloud_transform(&self) -> Transform { self.transform }

	pub fn to_sprite_bundles(
		&self,
		mut meshes: ResMut<Assets<Mesh>>,
		mut land_materials: ResMut<Assets<LandRiversShader>>,
		mut cloud_materials: ResMut<Assets<CloudShader>>
	) -> (
		MaterialMesh2dBundle<LandRiversShader>,
		MaterialMesh2dBundle<CloudShader>
	) {
		(
			MaterialMesh2dBundle {
				mesh: meshes.add(Rectangle::default()).into(),
				transform: self.land_rivers_transform(),
				material: land_materials.add(self.land_shader.clone()),
				..default()
			},
			MaterialMesh2dBundle {
				mesh: meshes.add(Rectangle::default()).into(),
				transform: self.cloud_transform(),
				material: cloud_materials.add(self.cloud_shader.clone()),
				..default()
			}
		)
	}
}
